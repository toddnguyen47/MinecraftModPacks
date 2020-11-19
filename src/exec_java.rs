use std::path::PathBuf;
use std::process::Command;

pub struct ExecJava;

impl ExecJava {
  pub fn execute_command(launcher_directory: &str) {
    let mut java_path = PathBuf::from(launcher_directory);
    java_path = PathBuf::from(java_path.parent().unwrap());
    java_path.push("runtime");
    java_path.push("jre-x64");
    java_path.push("bin");

    println!("{:?}", java_path);

    let output = if cfg!(target_os = "windows") {
      java_path.push("java.exe");
      Command::new(java_path)
        .args(&["-version"])
        .output()
        .expect("failed to execute process")
    } else {
      java_path.push("java");
      Command::new(java_path)
        .args(&["-version"])
        .output()
        .expect("failed to execute process")
    };

    let stdout = output.stdout;
    println!(
      "Output: '{}'",
      String::from_utf8(stdout).unwrap_or(String::from("ERROR: No output found"))
    );
  }
}
