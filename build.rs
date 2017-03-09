extern crate cmake;

use cmake::Config;

const CMAKE_AVRO_BUILD_PATH: &'static str = "/build/src";
const AVRO_SRC: &'static str = "avro/lang/c";

fn main() {
	let dst = Config::new(AVRO_SRC)
                 .define("CMAKE_INSTALL_PREFIX", "$PREFIX")
                 .define("CMAKE_BUILD_TYPE", "RelWithDebInfo")
                 .build();
    println!("cargo:rustc-link-search=all={}{}", dst.display(), CMAKE_AVRO_BUILD_PATH);
    println!("cargo:rustc-link-lib=static=avro");
}