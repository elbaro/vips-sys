use std::path::PathBuf;
use std::process::Command;
use failure::Error;

// borrowed from pkg-config crate
fn split_flags(output: &[u8]) -> Vec<String> {
    let mut word = Vec::new();
    let mut words = Vec::new();
    let mut escaped = false;

    for &b in output {
        match b {
            _ if escaped => {
                escaped = false;
                word.push(b);
            }
            b'\\' => {
                escaped = true
            }
            b'\t' | b'\n' | b'\r' | b' ' => {
                if !word.is_empty() {
                    words.push(String::from_utf8(word).unwrap());
                    word = Vec::new();
                }
            }
            _ => word.push(b),
        }
    }

    if !word.is_empty() {
        words.push(String::from_utf8(word).unwrap());
    }

    words
}

fn run(mut cmd: Command) -> Result<Vec<String>, Error> {
	let output = cmd.output()?;
	if output.status.success() {
		Ok(split_flags(&output.stdout[..]))
	} else {
		Err(failure::err_msg("pkg-config failed"))
	}
}

fn generate_bindings_rs() {
	let bindings = {

		let mut cmd = Command::new("pkg-config");
		cmd.args(&["--cflags", "vips"]);
		let flags = run(cmd).expect("Couldn't run pkg-config"); // include_dir flags required for vips.h


		let mut builder = bindgen::Builder::default()
			.header("wrapper.h")
			.rustified_enum(".*")
			.blacklist_type("max_align_t")
			.blacklist_type("FP_NAN")
			.blacklist_type("FP_INFINITE")
			.blacklist_type("FP_ZERO")
			.blacklist_type("FP_SUBNORMAL")
			.blacklist_type("FP_NORMAL")
			;

		for flag in flags.into_iter() {
			builder = builder.clang_arg(flag);
		}

		builder
	}
	.generate()
	.expect("Unable to generate bindings");

	let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("binding.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
	generate_bindings_rs();

	#[cfg(target_env = "msvc")]
	{
		vcpkg::find_package("vips").unwrap();
	}

	#[cfg(not(target_env = "msvc"))]
	{
		let mut config = pkg_config::Config::new();
		if cfg!(target_os="windows") || cfg!(target_os="macos") {
			// config.statik(true);
		}
		config.atleast_version("8.2").probe("vips").unwrap();
	}
}
