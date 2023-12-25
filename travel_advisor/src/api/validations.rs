pub fn string_to_id(str: String) -> Result<i64, String> {
    let n = match str.parse::<i64>() {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };

    if n <= 0 {
        return Err("must be a positive number".to_string());
    }

    Ok(n)
}
