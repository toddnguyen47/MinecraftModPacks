use std::fs::File;
use std::io::Read;

use yaml_rust::YamlLoader;

pub struct LoadYaml<'a> {
  file_path: &'a str,
  windows_path: String,
  mac_path: String,
}

impl<'a> LoadYaml<'a> {
  pub fn new(file_path: &str) -> LoadYaml {
    LoadYaml {
      file_path,
      windows_path: String::new(),
      mac_path: String::new(),
    }
  }

  pub fn execute(&mut self) {
    let mut file = File::open(self.file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let first_doc = &docs[0];
    self.windows_path = String::from(first_doc["windows_install_path"].as_str().unwrap());
    self.mac_path = String::from(first_doc["mac_install_path"].as_str().unwrap());

    println!("Windows: '{}'", self.windows_path);
    println!("Mac: '{}'", self.mac_path);
  }
}
