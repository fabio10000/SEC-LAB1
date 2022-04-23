use infer;

// DO NOT READ FILE CONTENTS INSIDE THIS FUNCTION
pub fn validate_file(file: &[u8]) -> bool {
    if (!infer::is_image(&file) && ! infer::is_video(file)) {
      return false
    }

    // todo: vérifier que mime type correspond à l'extension
    // let kind = infer::get(&file).expect("file type is known");
    // let mime = kind.mime_type();
    // let extension = kind.extension();
    true
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
