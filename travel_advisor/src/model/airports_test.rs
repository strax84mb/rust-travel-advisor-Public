#[cfg(test)]
mod com_tests {
    use serde::{Deserialize, Serialize};

    use crate::model::Airport;
    use super::super::airports::arrays::Airports;

    #[test]
    fn test_index() {
        let mut q = Airports::new();
        q <<= Airport {id: 1, name: "1".to_string(), city_id: 1};
        q <<= Airport {id: 2, name: "2".to_string(), city_id: 2};
        assert_eq!(2, q.len());
        let a = &q[0];
        assert_eq!(1, a.id.clone());
        let a = q.iter().position(|x| x.id == 2);
        assert!(a.is_some());
        assert_eq!(1, a.unwrap());
        q -= 1;
        assert_eq!(1, q.len());
        let a = &q[0];
        assert_eq!(2, a.id);
    }

    #[derive(Deserialize, Serialize)]
    struct TestSer {
        #[serde(rename(serialize="firstName", deserialize="firstName"))]
        first_field: String,
        #[serde(rename(serialize="secondName", deserialize="secondName"))]
        second_field: i16,
    }

    #[test]
    fn test_custom_filed_names() {
        let ts = TestSer {
            first_field: "qwerty".to_string(),
            second_field: 11,
        };
        let json = serde_json::to_string(&ts).unwrap();
        assert_eq!("{\"firstName\":\"qwerty\",\"secondName\":11}".to_string(), json);
        let json = "{\"firstName\":\"asd\",\"secondName\":22}";
        let ts: TestSer = serde_json::from_str(json).unwrap();
        assert_eq!("asd", ts.first_field);
        assert_eq!(22, ts.second_field);
    }

    fn get_param_name_by_type(param_type: &str, load: String) -> Option<String> {
        match load.find(param_type) {
            None => None,
            Some(pos) => {
                let mut temp: String = load.chars().skip(0).take(pos).collect();
                let start_pos = match temp.rfind(',') {
                    None => temp.rfind('(').expect("not even first param"),
                    Some(pos1) => pos1,
                };
                temp = temp.chars().skip(start_pos + 1).take_while(|x| (*x).ne(&':')).collect();
                Some(temp.trim().to_string())
            },
        }
    }
    
    #[test]
    fn test_auth_macro() {
        let func_str = "async fn upload_airpots(
            req: HttpRequest,
            payload: web::Bytes,
            airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
            auth_service: Data<Arc<dyn AuthService + Send + Sync>>
        ) -> Result<impl Responder, Error> {
            // validate access right
            //get_user_if_has_roles!(req, auth_service, vec![\"admin\"]);
            // save airports
            match airport_service.into_inner().save_airports(payload.to_vec().as_slice()) {
                Ok(()) => Ok(HttpResponse::Ok().finish()),
                Err(err) => Err(err.wrap_str(\"failed to save all airports\")),
            }
        }".to_string();
        let roles = "\"admin\"".to_string();
        let auth_service_param = get_param_name_by_type("Data<Arc<dyn AuthService + Send + Sync>>", func_str.clone());
        let req_param = get_param_name_by_type("HttpRequest", func_str.clone());
        let (first, other) = func_str.split_at(func_str.find('(').expect("failed to split string") + 1);
        let (second, third) = other.split_at(other.find('{').expect("failed to get split string the second time") + 1);
        let validation_code = format!(
            "match {}.get_user_if_has_role(
                match {}.headers().get(actix_web::http::header::AUTHORIZATION) {{
                    Some(header) => Some(header.to_str()),
                    None => None,
                }},
                vec![{}]
            ) {{
                Err(err) => return Err(err),
                Ok(user_option) => match user_option {{
                    Some(user) => user,
                    None => return Err(Error::unauthorized_str(\"user has no rights for this operation\")),
                }}
            }};",
            match auth_service_param.clone() {
                Some(s) => s,
                None => "auth_service".to_string(),
            },
            match req_param.clone() {
                Some(s) => s,
                None => "req".to_string(),
            },
            roles,
        );
        let final_func = format!(
            "{}{}{}{}{}{}",
            first,
            match req_param {
                None => "req: actix_web::HttpRequest,",
                _ => "",
            },
            match auth_service_param {
                None => "auth_service: actix_web::web::Data<Arc<dyn AuthService + Send + Sync>>,",
                _ => "",
            },
            second,
            validation_code,
            third,
        );
        println!("{}", final_func);    
    }
}