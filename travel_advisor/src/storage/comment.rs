pub mod comments {
    use std::{
        time::SystemTime,
        sync::Arc,
    };

    use diesel::{
        prelude::*,
        sql_function,
        insert_into,
        update,
        delete,
    };

    use crate::{
        model::Comment,
        schema::comments::dsl as comm_dsl,
        util::{
            Error,
            ErrorCode::{
                DbRead,
                DbSave,
                DbDelete,
                GetDbConnection,
            },
        },
        storage::{
            Database,
            db_context::db_macros::get_connection_v2,
            entities::InsertCommentDB,
        },
    };
    use super::super::entities::CommentDB;

    pub trait CommentRepository {
        fn create(&self, comment: Comment) -> Result<Comment, Error>;
        fn get_by_city(&self, city_id: i64) -> Result<Vec<Comment>, Error>;
        fn get_by_user(&self, user_id: i64) -> Result<Vec<Comment>, Error>;
        fn update(&self, id: i64, text: String) -> Result<(), Error>;
        fn delete(&self, id: i64) -> Result<(), Error>;
        fn delete_for_city(&self, city_id: i64) -> Result<(), Error>;
        fn get_by_id(&self, id: i64) -> Result<Option<Comment>, Error>;
    }

    struct CommentRepositoryImpl {
        db: Arc<Database>,
    }

    pub fn new_comment_repository(db: Arc<Database>) -> Arc<impl CommentRepository> {
        Arc::new(CommentRepositoryImpl {
            db: db,
        })
    }

    sql_function! { fn last_insert_id() -> BigInt; }

    impl CommentRepository for CommentRepositoryImpl {

        fn create(&self, comment: Comment) -> Result<Comment, Error> {
            let conn = &mut get_connection_v2!(self.db);
            let trx_result = conn.transaction::<i64, diesel::result::Error, _>(|conn| {
                let entity = InsertCommentDB {
                    text: comment.content.clone(),
                    city_id: comment.city_id.clone(),
                    user_id: comment.user_id.clone(),
                };
                match insert_into(comm_dsl::comments)
                    .values(&entity)
                    .execute(conn) {
                        Err(err) => Err(err),
                        Ok(_) => match comm_dsl::comments.select(last_insert_id()).load::<i64>(conn) {
                            Err(err) => Err(err),
                            Ok(ids) if ids.len() > 0 => Ok(ids[0]),
                            Ok(_) => Ok(-1),
                        },
                    }
            });
            match trx_result {
                Ok(id) => Ok(Comment {
                    id: id,
                    city_id: comment.city_id.clone(),
                    user_id: comment.user_id.clone(),
                    content: comment.content.clone(),
                    created_at: SystemTime::now(),
                    updated_at: SystemTime::now(),
                }),
                Err(err) => Err(Error::internal(DbSave, err.to_string())),
            }
        }
    
        fn get_by_city(&self, city_id: i64) -> Result<Vec<Comment>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            match comm_dsl::comments
                .filter(comm_dsl::city_id.eq(city_id))
                .select(CommentDB::as_select())
                .load(conn) {
                    Ok(result) => Ok(result.iter().map(|c| c.to_model()).collect()),
                    Err(err) => Err(Error::internal(DbRead, err.to_string())),
                }
        }
        
        fn get_by_user(&self, user_id: i64) -> Result<Vec<Comment>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            match comm_dsl::comments
                .filter(comm_dsl::user_id.eq(user_id))
                .select(CommentDB::as_select())
                .load(conn) {
                    Ok(result) => Ok(result.iter().map(|c| c.to_model()).collect()),
                    Err(err) => Err(Error::internal(DbRead, err.to_string())),
                }
        }
    
        fn update(&self, id: i64, text: String) -> Result<(), Error> {
            let conn = &mut get_connection_v2!(self.db);
            match update(comm_dsl::comments)
                .filter(comm_dsl::id.eq(id))
                .set(comm_dsl::text.eq(text))
                .execute(conn) {
                    Err(err) => Err(Error::internal(DbSave, err.to_string())),
                    Ok(result) if result > 0 => Ok(()),
                    _ => Err(Error::not_found("comment not found".to_string())),
                }
        }
    
        fn delete(&self, id: i64) -> Result<(), Error> {
            let conn = &mut get_connection_v2!(self.db);
            match delete(comm_dsl::comments)
                .filter(comm_dsl::id.eq(id))
                .execute(conn) {
                    Err(err) => Err(Error::internal(DbDelete, err.to_string())),
                    Ok(result) if result > 0 => Ok(()),
                    _ => Err(Error::not_found("comment not found".to_string())),
                }
        }
        
        fn delete_for_city(&self, city_id: i64) -> Result<(), Error> {
            let conn = &mut get_connection_v2!(self.db);
            match delete(comm_dsl::comments)
                .filter(comm_dsl::city_id.eq(city_id))
                .execute(conn) {
                    Err(err) => Err(Error::internal(DbDelete, err.to_string())),
                    Ok(_result) => Ok(()),
                }
        }
    
        fn get_by_id(&self, id: i64) -> Result<Option<Comment>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            match comm_dsl::comments
                .find(id)
                .select(CommentDB::as_select())
                .first(conn)
                .optional() {
                    Ok(result_opt) => match result_opt {
                        Some(rezz) => Ok(Some(rezz.to_model())),
                        None => Ok(None),
                    },
                    Err(err) => Err(Error::internal(DbRead, err.to_string())),
                }
        }
                
    }

}
