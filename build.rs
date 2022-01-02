use std::process::Command;
use std::path::Path;

fn main() {
	let build_dir = "./build";

	Command::new("mkdir")
					.args(&["-p"])
					.arg(&format!("{}", build_dir))
					.status().unwrap();

    Command::new("aarch64-none-elf-as")
					.args(&["src/boot.s", "-o"])
					.arg(&format!("{}/boot.o", build_dir))
        			.status().unwrap();

	Command::new("aarch64-none-elf-ar")
					.args(&["crus", "libboot.a", "boot.o"])
					.current_dir(&Path::new(&build_dir))
					.status().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=src/boot.s");
	println!("cargo:rustc-link-search=native={}", build_dir);
    println!("cargo:rustc-link-lib=static=boot");
}