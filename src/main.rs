use yaml_rust::YamlLoader;

use std::fs::File;
use std::io::prelude::Read;

fn main() {
  let file_name = "config.yaml";
  let mut file = File::open(file_name).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let docs = YamlLoader::load_from_str(&contents).unwrap();
  let first_doc = &docs[0];
  let windows_install_path = first_doc["windows_install_path"].as_str().unwrap();
  let mac_install_path = first_doc["mac_install_path"].as_str().unwrap();

  println!("Windows: '{}'", windows_install_path);
  println!("Mac: '{}'", mac_install_path);
}
