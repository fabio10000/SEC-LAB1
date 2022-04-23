use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use std::fs;
use std::path::Path;
use infer;
use read_input::prelude::*;
use lab01_2022_input_validation::*;
use uuid::Uuid;

static IMAGES_PATH: &str = "sec.upload/images/";
static VIDEOS_PATH: &str = "sec.upload/videos/";

struct StoredFile {
  path: String,
  file_type: String,
}

// Use the hashmap as follows:
// ```
// let map = HASHMAP.lock().unwrap();
// ```
lazy_static! {
  static ref HASHMAP: Mutex<HashMap<String, StoredFile>> = Mutex::new(HashMap::new());
}

fn file_upload(input: &String) -> Result<String, String> {
  if !fs::metadata(input).is_ok() {
    return Err("Invalid file !".to_string());
  }

  let file = match fs::read(input) {
    Ok(bytes) => bytes,
    Err(_) => return Err("Invalid file contents !".to_string()),
  };

  if !validate_file(&file, input) {
    return Err("Invalid file format !".to_string());
  }

  let mut map = HASHMAP.lock().unwrap();
  let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, &file).to_string();
  
  if !map.contains_key(&uuid) {
    let path;
    let file_type;
    let file_name = Path::new(&input).file_name().unwrap();
    if infer::is_image(&file) {
      path = format!("{}{}", IMAGES_PATH, file_name.to_str().unwrap());
      file_type = "an image".to_string();
    } else {
      path = format!("{}{}", VIDEOS_PATH, file_name.to_str().unwrap());
      file_type = "a video".to_string();
    }
    map.insert(uuid.clone(), StoredFile {
      path,
      file_type,
    });
    return Ok(format!("File uploaded successfully, UUID : {}", uuid));
  }
  return Err(format!("This file has already been uploaded with UUID : {}", uuid));
}

fn file_upload_handler() {
  loop {
    let input = input::<String>().repeat_msg("Please enter the path to an image or video file : ").get();
    match file_upload(&input) {
      Ok(msg) => {
        println!("{}", msg);
        break;
      },
      Err(msg) => println!("{}", msg)
    }
  }
}

fn file_verify(uuid: &String) -> Result<String, String> {
  let map = HASHMAP.lock().unwrap();
  if !map.contains_key(uuid) {
    return Err("No file corresponding to this UUID !".to_string());
  }

  let file = &map[uuid];
  return Ok(format!("File {} exists, it is {} file.", uuid, file.file_type));
}

fn file_verify_handler() {
  let input = input::<String>().repeat_msg("Please enter the UUID to check :")
  .add_err_test(|uuid| validate_uuid(uuid), "Invalid UUID !")
  .get();

  match file_verify(&input) {
    Ok(msg) => println!("{}", msg),
    Err(msg) => println!("{}", msg)
  }
}

fn get_url_handler() {
  let input = input::<String>().repeat_msg("Please enter the UUID to get :")
  .add_err_test(|uuid| validate_uuid(uuid), "Invalid UUID !")
  .get();

  let map = HASHMAP.lock().unwrap();
  match map.get(&input) {
    Some(file) => println!("{}", file.path),
    None => println!("No file corresponding to this UUID !")
  }
}

fn main() {
    println!("Welcome to the super secure file upload tool !");
    loop {
        match input::<i32>().repeat_msg("Please select one of the following options to continue :\n1 - Upload a file\n2 - Verify file exists\n3 - Get file URL\n0 - Exit\nYour input ? [0-3]")
            .min_max(0, 3).get() {
            0 => {
                println!("Goodbye!");
                break
            },
            1 => file_upload_handler(),
            2 => file_verify_handler(),
            3 => get_url_handler(),
            _ => panic!("Invalid input"),
        }
    }
}
