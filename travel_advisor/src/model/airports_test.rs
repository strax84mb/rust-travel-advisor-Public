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
        let func_str = "async fn upload_cities(
            req: HttpRequest,
            payload: web::Bytes,
            auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
            city_service: Data<Arc<dyn CityService + Send + Sync>>,
        ) -> impl Responder {
            match city_service.into_inner().save_cities(payload.to_vec().as_slice()) {
                Ok(()) => respond_ok(Some(\"saved all cities\")),
                Err(err) => resolve_error(err, Some(\"failed to save all cities\")),
            }
        }".to_string();
        let roles = "\"admin\"";
        let auth_service_param = get_param_name_by_type("Data<Arc<dyn AuthService + Send + Sync>>", func_str.clone());
        let req_param = get_param_name_by_type("HttpRequest", func_str.clone());
        let (first, other) = func_str.split_at(func_str.find('(').expect("failed to get start of attributes") + 1);
        let (second, third) = other.split_at(other.find('{').expect("failed to get start of attributes") + 1);
        let validation_code = format!(
            "match {}.has_role(extract_auth(&{}), vec![{}]) {{
                Ok(found) if !found => return respond_unauthorized(None),
                Err(err) => return resolve_error(err, Some(\"failed to check JWT\")),
            _   => (),
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
                None => "req: HttpRequest,",
                _ => "",
            },
            match auth_service_param {
                None => "auth_service: Data<Arc<dyn AuthService + Send + Sync>>,",
                _ => "",
            },
            second,
            validation_code,
            third,
        );
        println!("{}", final_func);
    }
}