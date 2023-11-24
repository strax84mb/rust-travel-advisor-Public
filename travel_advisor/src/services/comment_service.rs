pub mod services {
    use std::sync::Arc;

    use async_trait::async_trait;
    use log::error;

    use crate::{
        CommentRepository,
        model::{
            Comment,
            User,
        },
        util::app_errors::Error,
    };
    use super::super::traits::CommentService;

    struct CommentServiceImpl {
        repo: Arc<dyn CommentRepository + Sync + Send>,
    }

    pub fn new_comment_service(repo: Arc<dyn CommentRepository + Sync + Send>) -> Arc<impl CommentService> {
        Arc::new(CommentServiceImpl {
            repo: repo,
        })
    }

    #[async_trait]
    impl CommentService for CommentServiceImpl {

        async fn create(&self, user_id: i64, mut comment: Comment) -> Result<Comment, Error> {
            comment.user_id = user_id;

            match self.repo.create(comment).await {
                Ok(comment) => Ok(comment),
                Err(err) => {
                    error!("failed to save comment: {}", err.to_string());
                    Err(Error::wrap_str(err, "failed to save comment"))
                }
            }
        }

        async fn update(&self, user_id: i64, comment: Comment) -> Result<Comment, Error> {
            if user_id.clone() != comment.user_id.clone() {
                return Err(Error::forbidden())
            }
            // update comment
            match self.repo.update(comment.id.clone(), comment.content.clone()).await {
                Ok(()) => (),
                Err(err) => {
                    error!("failed to update comment: {}", err.to_string());
                    return Err(Error::wrap_str(err, "failed to update comment"));
                },
            }
            // reload comment
            match self.repo.get_by_id(comment.id.clone()).await {
                Ok(comment) => match comment {
                    Some(comment) => Ok(comment),
                    None => panic!("this should never happen"),
                },
                Err(err) => {
                    error!("failed to reload comment: {}", err.to_string());
                    Err(Error::wrap_str(err, "failed to reload comment"))
                },
            }
        }

        async fn delete(&self, id: i64, user: User) -> Result<(), Error> {
            let allowed = user.is_admin();
            let comment = match self.repo.get_by_id(id).await {
                Ok(comment) => match comment {
                    Some(comment) => comment,
                    None => return Err(Error::not_found()),
                },
                Err(err) => {
                    error!("failed to load comment: {}", err.to_string());
                    return Err(Error::wrap_str(err, "failed to load comment"));
                },
            };

            if !allowed && comment.user_id.clone() != user.id.clone() {
                return Err(Error::forbidden())
            }

            match self.repo.delete(id).await {
                Ok(()) => Ok(()),
                Err(err) => {
                    error!("failed to delete comment: {}", err.to_string());
                    Err(Error::wrap_str(err, "failed to delete comment"))
                },
            }
        }

        async fn list_for_city(&self, city_id: i64) -> Result<Vec<Comment>, Error> {
            match self.repo.get_by_city(city_id).await {
                Ok(result) => Ok(result),
                Err(err) => {
                    error!("failed to list comments for city: {}", err.to_string());
                    Err(err)
                },
            }
        }

        async fn list_for_user(&self, user_id: i64) -> Result<Vec<Comment>, Error> {
            match self.repo.get_by_user(user_id).await {
                Ok(result) => Ok(result),
                Err(err) => {
                    error!("failed to list comments of the user: {}", err.to_string());
                    Err(err)
                },
            }
        }

        async fn get_by_id(&self, id: i64) -> Result<Option<Comment>, Error> {
            match self.repo.get_by_id(id).await {
                Ok(comment) => Ok(comment),
                Err(err) => {
                    error!("failed to get comment by ID: {}", err.to_string());
                    Err(err)
                },
            }
        }

    }

}