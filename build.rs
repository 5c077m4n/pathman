use std::{env, fs, io::{self, Write}, path::Path};

fn main() -> io::Result<()> {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("pj.sh");

    let mut file = fs::File::create(&dest_path)?;
	file.write_all(
		b"
        #!/bin/sh

        ./pj
        alias ${PJ_CMD:-pj}='./pj --add \"$1\" && cd \"$_\"'
        ",
	)?;

	println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}