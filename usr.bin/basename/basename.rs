use std::env;
use std::process::exit;

pub fn basename<'n>(name: &'n str, suffix: &str) -> &'n str {
	let mut target = name;

	if suffix.len() > 0 {
		target = name.strip_suffix(suffix).unwrap_or(name);
	}

	let result: Vec<&str> = target
		.strip_suffix("/")
		.unwrap_or(target)
		.split("/")
		.collect();

	result.last().unwrap()
}

fn main() {
	let mut args: Vec<String> = env::args().collect();
	args.drain(0..1);

	if args.len() < 1 {
		println!("basename: error: missing operand");
		exit(1);
	}

	if args.len() > 2 {
		println!("basename: error: extra operands");
		exit(1);
	}

	let name = &args[0];
	let suffix: &str;

	if args.len() == 2 {
		suffix = &args[1]
	} else {
		suffix = ""
	}

	println!("{}", basename(&name, suffix));
	exit(0);
}