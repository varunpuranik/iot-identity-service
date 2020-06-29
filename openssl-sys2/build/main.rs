#![deny(rust_2018_idioms, warnings)]

fn main() {
	openssl_build::define_version_number_cfg();

	let mut build = openssl_build::get_c_compiler();
	build.file("build/compat.c").compile("openssl_sys2_compat_wrapper");
}
