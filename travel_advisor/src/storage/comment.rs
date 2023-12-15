pub mod comments {
    use std::{
        time::{
            SystemTime,
            UNIX_EPOCH,
        },
        sync::Arc,
    };

    use async_trait::async_trait;
    use sqlx::{
        Row,
        mysql::MySqlRow,
        FromRow,
        types::chrono::{
            DateTime,
            NaiveDateTime,
            Utc,
        },
    };

    use crate::{
        model::Comment,
        util::app_errors::Error, storage::Database,
    };

    #[async_trait]
    pub trait CommentRepository {
        async fn create(&self, comment: Comment) -> Result<Comment, Error>;
        async fn get_by_city(&self, city_id: i64) -> Result<Vec<Comment>, Error>;
        async fn get_by_user(&self, user_id: i64) -> Result<Vec<Comment>, Error>;
        async fn update(&self, id: i64, text: String) -> Result<(), Error>;
        async fn delete(&self, id: i64) -> Result<(), Error>;
        async fn delete_for_city(&self, city_id: i64) -> Result<(), Error>;
        async fn get_by_id(&self, id: i64) -> Result<Option<Comment>, Error>;
    }

    struct CommentRepositoryImpl {
        db: Arc<Database>,
    }

    pub fn new_comment_repository(db: Arc<Database>) -> Arc<impl CommentRepository> {
        Arc::new(CommentRepositoryImpl {
            db: db,
        })
    }

    impl CommentRepositoryImpl {
    
        fn current_date_time(&self) -> Result<DateTime<Utc>, Error> {
            let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(now) => now.as_nanos(),
                Err(err) => return Err(Error::underlying(format!("failed to get current time: {}", err.to_string()))),
            };
            let secs = (now.clone()/1000000) as i64;
            let nanos = (now.clone() % 1000000) as u32;
            let now = match NaiveDateTime::from_timestamp_opt(secs, nanos) {
                Some(now) => now,
                None => return Err(Error::from_str("failed to construct naive date time")),
            };
            let now: DateTime<Utc> = now.and_utc();
            Ok(now)
        }

        fn handle_rows(&self, rows: Result<Vec<MySqlRow>, sqlx::Error>) -> Result<Vec<Comment>, Error> {
            match rows {
                Ok(rows) => {
                    let comments: Result<Vec<Comment>, sqlx::Error> = rows.iter()
                        .map(|row| Comment::from_row(row))
                        .collect();
                    match comments {
                        Ok(comments) => Ok(comments),
                        Err(err) => Err(Error::underlying(format!("failed to parse query results: {}", err.to_string()))),
                    }
                },
                Err(err) => Err(Error::underlying(format!("failed to execute query: {}", err.to_string()))),
            }
        }
        
    }

    #[async_trait]
    impl CommentRepository for CommentRepositoryImpl {

        async fn create(&self, comment: Comment) -> Result<Comment, Error> {
            let now = match self.current_date_time() {
                Ok(now) => now,
                Err(err) => return Err(Error::underlying(format!("failed to timestamp: {}", err.to_string()))),
            };
            // begin transaction
            let mut tx = match self.db.connections.as_ref().begin().await {
                Ok(t) => t,
                Err(err) => return Err(Error::underlying(err.to_string())),
            };
    
            let statement = sqlx::query(
                "INSERT INTO comments (city_id, user_id, `text`, created_at, updated_at) VALUES (?, ?, ?, ?, ?)")
                .bind(comment.city_id.clone())
                .bind(comment.user_id.clone())
                .bind(comment.content.clone())
                .bind(now.clone())
                .bind(now.clone())
                .execute(&mut tx);
            let result: Result<i64, Error> = match statement.await {
                Ok(result) => {
                    if result.rows_affected() == 0 {
                        Err(Error::underlying("No row inserted".to_string()))
                    } else {
                        match sqlx::query("SELECT LAST_INSERT_ID()").fetch_one(&mut tx).await {
                            Ok(row) => {
                                let id: i64 = match row.try_get(0) {
                                    Ok(v) => v,
                                    Err(err) => return Err(Error::underlying(err.to_string())),
                                };
    
                                Ok(id)
                            },
                            Err(err) => Err(Error::underlying(err.to_string())),
                        }
                    }
                },
                Err(err) => Err(Error::underlying(err.to_string())),
            };
    
            let mut comment = comment.clone();
            if result.is_ok() {
                comment.id = result.unwrap();
                
                match tx.commit().await {
                    Ok(()) => Ok(comment),
                    Err(err) => Err(Error::underlying(format!("failed to commit: {}", err.to_string()))),
                }
            } else {
                let err = result.err().unwrap();
                match tx.rollback().await {
                    Ok(()) => Err(err),
                    Err(err2) => return Err(Error::underlying(
                        format!("failed to rollback: {}, for error: {}", err2.to_string(), err.to_string())
                    )),
                }
            }
        }
    
        async fn get_by_city(&self, city_id: i64) -> Result<Vec<Comment>, Error> {
            let statement = sqlx::query("SELECT id, city_id, user_id, `text`, created_at, updated_at FROM comments WHERE city_id = ?")
                .bind(city_id)
                .fetch_all(self.db.connections.as_ref())
                .await;
            self.handle_rows(statement)
        }
        
        async fn get_by_user(&self, user_id: i64) -> Result<Vec<Comment>, Error> {
            let statement = sqlx::query("SELECT id, city_id, user_id, `text`, created_at, updated_at FROM comments WHERE user_id = ?")
                .bind(user_id)
                .fetch_all(self.db.connections.as_ref())
                .await;
            self.handle_rows(statement)
        }
    
        async fn update(&self, id: i64, text: String) -> Result<(), Error> {
            let now = match self.current_date_time() {
                Ok(now) => now,
                Err(err) => return Err(Error::underlying(format!("failed to timestamp: {}", err.to_string()))),
            };
            let statement = sqlx::query(
                "UPDATE comments SET `text` = ?, updated_at = ? WHERE id = ?"
                ).bind(text)
                .bind(now.clone())
                .bind(id)
                .execute(self.db.connections.as_ref());
            match statement.await {
                Ok(result) => {
                    if result.rows_affected() == 0 {
                        Err(Error::not_found())
                    } else {
                        Ok(())
                    }
                },
                Err(err) => Err(Error::underlying(format!("failed to execute statement: {}", err.to_string()))),
            }
        }
    
        async fn delete(&self, id: i64) -> Result<(), Error> {
            let statement = sqlx::query("DELETE FROM comments WHERE id = ?")
                .bind(id)
                .execute(self.db.connections.as_ref());
            match statement.await {
                Ok(result) => {
                    if result.rows_affected() == 0 {
                        Err(Error::not_found())
                    } else {
                        Ok(())
                    }
                },
                Err(err) => Err(Error::underlying(format!("failed to execute statement: {}", err.to_string()))),
            }
        }
        
        async fn delete_for_city(&self, city_id: i64) -> Result<(), Error> {
            let statement = sqlx::query("DELETE FROM comments WHERE city_id = ?")
                .bind(city_id)
                .execute(self.db.connections.as_ref());
            match statement.await {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::underlying(format!("failed to execute statement: {}", err.to_string()))),
            }
        }
    
        async fn get_by_id(&self, id: i64) -> Result<Option<Comment>, Error> {
            let statement = sqlx::query(
                "SELECT id, city_id, user_id, `text`, created_at, updated_at FROM comments WHERE id = ?"
                ).bind(id)
                .fetch_optional(self.db.connections.as_ref());
            match statement.await {
                Ok(row) => match row {
                    Some(row) => match Comment::from_row(&row) {
                        Ok(comment) => Ok(Some(comment)),
                        Err(err) => Err(Error::underlying(format!("failed to parse query results: {}", err.to_string()))),
                    },
                    None => Ok(None),
                },
                Err(err) => Err(Error::underlying(format!("failed to execute query: {}", err.to_string()))),
            }
        }
                
    }

}
