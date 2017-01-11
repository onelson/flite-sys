use std::process::Command;

fn main() {
    let src_dir = "./flite-2.0.0-release";
    let out_dir = "./flite-2.0.0-release/build/x86_64-linux-gnu/lib";

    Command::new("tar").args(&["xf", "flite-2.0.0-release.tar.bz2"]).status().unwrap();
    Command::new("sh").args(&["configure", "--with-pic"])
        .current_dir(&src_dir)
        .status().unwrap();
    Command::new("make")
        .current_dir(&src_dir)
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=flite");
}
