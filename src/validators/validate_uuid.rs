use regex::Regex;
use lazy_static::lazy_static;

// DO NOT READ FILE CONTENTS INSIDE THIS FUNCTION
pub fn validate_uuid(uuid_str: &String) -> bool {
  lazy_static! {
    // https://en.wikipedia.org/wiki/Universally_unique_identifier#Format
    // regex based on the link above
    static ref RE: Regex = Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-5[0-9a-fA-F]{3}-[89aAbB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap();
  }

  return RE.is_match(uuid_str);
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
