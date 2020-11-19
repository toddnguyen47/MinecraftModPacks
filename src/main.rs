mod load_yaml;
use load_yaml::LoadYaml;

fn main() {
  let mut load_yaml_obj = LoadYaml::new("config.yaml");
  load_yaml_obj.execute();
}
