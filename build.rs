
fn main() {
  match std::env::var("EMS_HOME") {
    Ok(directory) => {
      println!("cargo:rustc-link-search=native={}", directory);
      println!("cargo:rustc-link-search=native={}/lib", directory);
      println!("cargo:rustc-link-search=native={}/lib/64", directory);
    },
    Err(_err) => {}
  }
  println!("cargo:rerun-if-env-changed=EMS_HOME");
  println!("cargo:rustc-link-lib=dylib=tibems64");
  println!("cargo:rustc-link-lib=dylib=tibemsadmin64");
}