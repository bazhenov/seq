extern crate clap;

use clap::App;
mod processing;

fn main() {
	println!("Hello, world!");
}

fn app<'a, 'b>() -> App<'a, 'b> {
	App::new("sq")
		.author("Denis Bazhenov")
		.version("1.0")
		.about("sequence processing toolchain")
		.arg_from_usage("-f, --file=<FILE> 'Sets the input file'")
}
