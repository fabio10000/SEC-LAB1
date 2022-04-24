use infer;
use std::path::Path;

// DO NOT READ FILE CONTENTS INSIDE THIS FUNCTION
pub fn validate_file(file: &[u8], path: &String) -> bool {
    if !infer::is_image(&file) && ! infer::is_video(file) {
      return false
    }

    let kind = infer::get(&file).expect("file type is known");
    return match Path::new(path).extension().unwrap().to_str().unwrap() {
      "jpeg" => kind.extension() == "jpg",
      "tif" => kind.extension() == "tiff",
      extension => kind.extension() == extension
    };
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
