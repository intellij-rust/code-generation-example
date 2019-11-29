use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    generate_file(&dest_path, b"
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ");

    let gen_dir = Path::new(&out_dir).join("gen");
    if !gen_dir.exists() {
        fs::create_dir(&gen_dir).unwrap();
    }
    let dest_path2 = gen_dir.join("hello2.rs");
    generate_file(&dest_path2, b"
        pub fn message2() -> &'static str {
            \"Hello, World!\"
        }
    ");
    println!("cargo:rustc-env=GENERATED_ENV={}", gen_dir.display());
}

fn generate_file<P: AsRef<Path>>(path: P, text: &[u8]) {
    let mut f = File::create(path).unwrap();
    f.write_all(text).unwrap()
}
