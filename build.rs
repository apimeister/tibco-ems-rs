fn main() {
  println!("cargo:rustc-link-lib=dylib=tibems64");
  println!("cargo:rustc-link-lib=dylib=tibemsadmin64");
}