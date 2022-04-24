use infer;
use std::path::Path;

// DO NOT READ FILE CONTENTS INSIDE THIS FUNCTION
pub fn validate_file(file: &[u8], path: &String) -> bool {
    if !infer::is_image(&file) && ! infer::is_video(file) {
      return false
    }

    let kind = infer::get(&file).expect("file type is known");

    // translate special extensions supported by infer
    let extension = match Path::new(path).extension().unwrap().to_str().unwrap() {
      "jpeg" | "jpe" => "jpg",
      "tiff" => "tif",
      "qt" => "mov",
      other => other
    };

    return extension == kind.extension()
}

#[cfg(test)]
mod tests {
  use crate::validators::validate_file;

  #[test]
  fn it_should_validate_images_and_videos() {
    // source: https://en.wikipedia.org/wiki/List_of_file_signatures
    // magic bytes
    let jpeg = &[0xFF, 0xD8, 0xFF];
    let webm = &[0x1A, 0x45, 0xDF, 0xA3];
    let xml = b"<?xml";

    assert!(validate_file(webm, &"example.webm".to_string()));
    assert!(validate_file(jpeg, &"example.jpg".to_string()));

    // not an image or video file
    assert!(!validate_file(xml, &"example.xml".to_string()));

    // file format not supported
    let mpeg_video = &[0x00, 0x00, 0x01, 0xB3];
    assert!(!validate_file(mpeg_video, &"example.mpeg".to_string()));
  }

  #[test]
  fn mime_type_should_match_extension() {
    // source: https://en.wikipedia.org/wiki/List_of_file_signatures
    // magic bytes
    let jpeg = &[0xFF, 0xD8, 0xFF];
    let png = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    // with multiple extensions
    assert!(validate_file(jpeg, &"example.jpg".to_string()));
    assert!(validate_file(jpeg, &"example.jpeg".to_string()));
    assert!(!validate_file(jpeg, &"example.png".to_string()));

    assert!(validate_file(png, &"example.png".to_string()));
    assert!(!validate_file(png, &"example.tif".to_string()));
 
  }
}
