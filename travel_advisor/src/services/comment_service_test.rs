#[cfg(test)]
mod airport_service_test {

    use std::{sync::Arc, time::SystemTime};

    use async_trait::async_trait;
    use mockall::{
        mock,
        predicate::eq,
    };

    use crate::{
        model::{
            Comment,
        },
        util::app_errors::Error,
    };

    use crate::storage::CommentRepository;
    use super::super::{
        comment_service::services::new_comment_service,
        traits::CommentService,
    };

    mock! {

        pub CommentRepositoryTest {}

        #[async_trait]
        impl CommentRepository for CommentRepositoryTest {
            async fn create(&self, comment: Comment) -> Result<Comment, Error>;
            async fn get_by_city(&self, city_id: i64) -> Result<Vec<Comment>, Error>;
            async fn get_by_user(&self, user_id: i64) -> Result<Vec<Comment>, Error>;
            async fn update(&self, id: i64, text: String) -> Result<(), Error>;
            async fn delete(&self, id: i64) -> Result<(), Error>;
            async fn delete_for_city(&self, city_id: i64) -> Result<(), Error>;
            async fn get_by_id(&self, id: i64) -> Result<Option<Comment>, Error>;
        }

    }

    #[actix_rt::test]
    async fn create_comment() {
        let mut mock = MockCommentRepositoryTest::new();

        let now = SystemTime::now();

        mock.expect_get_by_id()
            .with(eq(1 as i64))
            .times(1)
            .return_once(move |_x| {
                Ok(Some(Comment {
                    id: 1,
                    city_id: 2,
                    user_id: 3,
                    content: "content".to_string(),
                    created_at: now.clone(),
                    updated_at: now.clone(),
                }))
            });

        let mock_param: Arc<dyn CommentRepository + Send + Sync> = Arc::new(mock);
        let service = new_comment_service(mock_param);

        let comment = service.get_by_id(1).await;

        assert_eq!(true, comment.is_ok());
        let comment = comment.unwrap();
        assert_eq!(true, comment.is_some());
        let comment = comment.unwrap();
        assert_eq!(1, comment.id);
        assert_eq!(2, comment.city_id);
        assert_eq!(3, comment.user_id);
        assert_eq!(true, "content".to_string().eq(&comment.content));
        assert_eq!(true, now.eq(&comment.created_at));
        assert_eq!(true, now.eq(&comment.updated_at));
    }

}