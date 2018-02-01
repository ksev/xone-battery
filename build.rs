use std::process::Command;

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let binary = r#"C:\Program Files (x86)\Windows Kits\10\bin\x64\rc.exe"#;
    
    Command::new(binary)
        .args(&["/r", "res/application.rc"])
        .output()
        .expect("Failed to build resource file");

    std::fs::create_dir_all(format!("target/{}/deps", profile)).expect("Could not create out dir");

    std::fs::rename("res/application.res", format!("target/{}/deps/application.lib", profile)).unwrap();

    println!("cargo:rustc-link-lib=static=application");
}