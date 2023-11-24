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
}