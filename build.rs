fn main() {
    if cfg!(feature = "ems-sys") {
        match std::env::var("EMS_HOME") {
            Ok(directory) => {
                println!("cargo:rustc-link-search=native={directory}");
                println!("cargo:rustc-link-search=native={directory}/lib/64");
                println!("cargo:rustc-link-search=native={directory}/lib");
            }
            Err(_err) => {}
        }
        println!("cargo:rerun-if-env-changed=EMS_HOME");
        println!("cargo:rustc-link-lib=dylib=tibems");
    }
}
