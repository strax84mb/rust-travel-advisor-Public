pub mod services {
    use std::sync::Arc;

    use crate::{
        model::Route,
        storage::routes::RouteRepository,
        util::Error,
    };
    use super::super::{
        macros::log_if_error,
        traits::RouteService,
    };


    struct RouteServiceImpl {
        route_repo: Arc<dyn RouteRepository + Sync + Send>
    }

    impl RouteService for RouteServiceImpl {
        fn get_all(&self, offset: i64, limit: i64) -> Result<Vec<Route>, Error> {
            log_if_error!(self.route_repo.get_all(offset, limit))
        }

        fn find_by_id(&self, id: i64) -> Result<Option<Route>, Error> {
            log_if_error!(self.route_repo.find_by_id(id))
        }

        fn update(&self, route: Route) -> Result<(), Error> {
            log_if_error!(self.route_repo.update(route))
        }

        fn delete(&self, id: i64) -> Result<(), Error> {
            log_if_error!(self.route_repo.delete(id))
        }

        fn save_routes(&self, sv_text: &[u8]) -> Result<(), Error> {
            Err(Error::forbidden_str(""))
        }

        fn find_cheapest_route(&self, start: i64, finish: i64) -> Result<Vec<Route>, Error> {
            Err(Error::forbidden_str(""))
        }
    }
}