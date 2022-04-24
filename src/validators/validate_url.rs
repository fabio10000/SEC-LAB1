/**
 * Author: Fabio da Silva Marques
 * Last edit: 24.04.2022
 */
use regex::Regex;
use lazy_static::lazy_static;

/// Verifies if it's a valid url using a simplist regex
/// # Arguments
/// * `url` - An URL as a string
/// * `tld_whitelist` - A list of whitelisted top level domains if an empty list is given then it's going to match all possible top level domains
pub fn validate_url(url: &String, tld_whitelist: &[String]) -> bool {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^([a-z\d]+://)?[a-zA-Z\d\-\.]+(?P<tld>\.[a-zA-Z\d\.]+[a-zA-Z])([/#].*)?$").unwrap();
  }

  return match RE.captures(url) {
    // if is a valid url then check if top-level-domain is present inside the whitelist
    Some(caps) => {
      if tld_whitelist.len() > 0 {
        return tld_whitelist.contains(&caps["tld"].to_string());
      } else {
        true
      }
    },
    None => false
  };
}

#[cfg(test)]
mod tests {
  use crate::validators::validate_url;
  #[test]
  fn it_works_matching_protocol() {
    // should work
    assert!(validate_url(&"http://example.com".to_string(), &[]));
    assert!(validate_url(&"x://example.com".to_string(), &[]));
    assert!(validate_url(&"2://example.com".to_string(), &[]));
    assert!(validate_url(&"ftp3://example.com".to_string(), &[]));
    assert!(validate_url(&"example.com".to_string(), &[]));

    // shouldn't work
    assert!(!validate_url(&"://example.com".to_string(), &[]));
    assert!(!validate_url(&"a:/example.com".to_string(), &[]));
    assert!(!validate_url(&"a/example.com".to_string(), &[]));
    assert!(!validate_url(&"a//example.com".to_string(), &[]));
    assert!(!validate_url(&"a:example.com".to_string(), &[]));
    assert!(!validate_url(&"&://example.com".to_string(), &[]));
    assert!(!validate_url(&"ftp+://example.com".to_string(), &[]));
  }

  #[test]
  fn it_works_matching_subdomain() {
    // should work
    assert!(validate_url(&"example.com".to_string(), &[]));
    assert!(validate_url(&"ex-ample.com".to_string(), &[]));
    assert!(validate_url(&"ex.amp-le.com".to_string(), &[]));
    assert!(validate_url(&"ex.am2p-le.com".to_string(), &[]));
    assert!(validate_url(&"eX.aM2p-lE.com".to_string(), &[]));
    assert!(validate_url(&"ex...amp-le..com".to_string(), &[]));
    assert!(validate_url(&"...com".to_string(), &[]));

    // shouldn't work
    assert!(!validate_url(&".&..com".to_string(), &[]));
  }

  #[test]
  fn it_works_matching_tld() {
    // should work
    assert!(validate_url(&"example.com".to_string(), &[]));
    assert!(validate_url(&"example.ch".to_string(), &[]));
    assert!(validate_url(&"example.de".to_string(), &[]));
    assert!(validate_url(&"example..a".to_string(), &[]));
    assert!(validate_url(&"example.a.a".to_string(), &[]));
    assert!(validate_url(&"example.aqwioequeioqweuowqieuoiwqeuwq".to_string(), &[]));
    assert!(validate_url(&"example.a2a".to_string(), &[]));

    // shouldn't work
    assert!(!validate_url(&"example.a".to_string(), &[]));
    assert!(!validate_url(&"example.aaaa.".to_string(), &[]));
    assert!(!validate_url(&"example.a*+".to_string(), &[]));
    assert!(!validate_url(&"example.".to_string(), &[]));
  }

  #[test]
  fn it_works_matching_whitelisted_tld() {
    let dot_com = [".com".to_string()];
    let many_tld = [".com".to_string(), ".net".to_string(), ".org".to_string(), ".invalid.".to_string(), ".pt.br".to_string()];
    // with one element
    assert!(validate_url(&"example.com".to_string(), &dot_com));
    assert!(!validate_url(&"example.other".to_string(), &dot_com));

    // many elements
    assert!(validate_url(&"example.com".to_string(), &many_tld));
    assert!(validate_url(&"example.net".to_string(), &many_tld));
    assert!(validate_url(&"example.com".to_string(), &many_tld));
    assert!(!validate_url(&"example.other".to_string(), &many_tld));
    assert!(!validate_url(&"example.invalid.".to_string(), &many_tld));

    // edge case
    // does not match because need a more specific regex to understand if is a subdomain or top level domain
    // it's possible to make the matching of subdomain non greedy to match that example
    // but the downside is that if we take for example "ex.ample.pt.br" the tld will be ".ample.pt.br"
    assert!(!validate_url(&"example.pt.br".to_string(), &many_tld));
  }

  #[test]
  fn it_works_matching_end_of_url() {
    // should work
    assert!(validate_url(&"example.com".to_string(), &[]));
    assert!(validate_url(&"example.com/".to_string(), &[]));
    assert!(validate_url(&"example.com/&%ç*+\"*".to_string(), &[]));
    assert!(validate_url(&"example.com#".to_string(), &[]));
    assert!(validate_url(&"example.com#&%$àéè".to_string(), &[]));

    // shouldn't work
    assert!(!validate_url(&"example.com&".to_string(), &[]));
    assert!(!validate_url(&"example.com%".to_string(), &[]));
    assert!(!validate_url(&"example.com-".to_string(), &[]));
  }
}
