pub mod sqlx_parse_utils {
  use sqlx::types::chrono;

  pub fn string_to_timestamp(target: &str) -> Result<chrono::NaiveDateTime, String> {
    let format = "%Y-%m-%d %H:%M:%S%.f %:z";

    match chrono::NaiveDateTime::parse_from_str(&target, format) {
      Ok(r) => Ok(r),
      Err(_) => return Err("Cant parse string to sqlx timestamp.".to_string()),
    }
  }

  pub fn option_string_to_timestamp(
    target: &Option<String>,
  ) -> Result<Option<chrono::NaiveDateTime>, String> {
    match target {
      Some(date_str) => Ok(Some(string_to_timestamp(&date_str).unwrap())),
      None => Ok(None),
    }
  }

  pub fn option_timestamp_to_string(
    target: &Option<chrono::NaiveDateTime>,
  ) -> Result<Option<String>, String> {
    match target {
      Some(date) => Ok(Some(date.to_string())),
      None => Ok(None),
    }
  }
}
