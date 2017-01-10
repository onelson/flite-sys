extern crate pkg_config;

fn main() {
    pkg_config::Config::new()
        .atleast_version("2.0.0").statik(true).probe("flite").unwrap();
}