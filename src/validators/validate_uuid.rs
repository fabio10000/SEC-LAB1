/**
 * Author: Fabio da Silva Marques
 * Last edit: 24.04.2022
 */
use regex::Regex;
use lazy_static::lazy_static;

/// Function that verifies if it's a valid UUID v5
/// doesn't check for the used variant
/// # Arguments
/// * `uuid_str` - An uuid to test as a string
pub fn validate_uuid(uuid_str: &String) -> bool {
  lazy_static! {
    // https://en.wikipedia.org/wiki/Universally_unique_identifier#Format
    // regex based on the link above
    static ref RE: Regex = Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-5[0-9a-fA-F]{3}-[89aAbB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap();
  }

  return RE.is_match(uuid_str);
}

#[cfg(test)]
mod tests {
  use crate::validators::validate_uuid;

  #[test]
  fn it_should_be_a_valid_uuid() {
    assert!(!validate_uuid(&"asdasdasdd".to_string()));
    assert!(!validate_uuid(&"aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa".to_string()));
    assert!(validate_uuid(&"5301963a-065a-5962-99d2-85846dd846ef".to_string()));
  }

  #[test]
  fn it_should_reject_wrong_versions() {
    // v1
    assert!(!validate_uuid(&"3518e3fe-c3e6-11ec-9d64-0242ac120002".to_string()));
    // v3
    assert!(!validate_uuid(&"f0df6355-29fa-36c4-89e4-c9a80042c866".to_string()));
    // v4
    assert!(!validate_uuid(&"e4e14ea4-dbaf-4726-a666-1333f6922b00".to_string()));
    // v5
    assert!(validate_uuid(&"5301963a-065a-5962-99d2-85846dd846ef".to_string()));
  }
}
