mod load_yaml;
use load_yaml::LoadYaml;

mod exec_java;
use exec_java::ExecJava;

fn main() {
  let mut load_yaml_obj = LoadYaml::new("config.yaml");
  load_yaml_obj.execute();
  ExecJava::execute_command(&(load_yaml_obj.get_windows_path()));
}
