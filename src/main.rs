use std::io::Write as _;
use clap::Parser;
use console::style;

mod macros;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(help = "The file to compile")]
	file: Option<String>,

	#[arg(short, long, help = "Enables debug outputs in compiled executable")]
	debug: bool,

	#[arg(short, long, help = "Runs directory from memory (disables stdin)")]
	run: bool,

	#[arg(short, long, help = "Display individual steps")]
	verbose: bool,
}

fn tcc_path() -> Result<std::path::PathBuf, String> {
	return match std::env::current_exe() {
		Ok(o) => match o.parent() {
			Some(o) => match o.join("tcc/tcc.exe").canonicalize() {
				Ok(o) => Ok(o),
				Err(e) => {
					return Err(e.to_string());
				}
			},
			None => {
				return Err("Could not get parent".to_string());
			}
		},
		Err(e) => {
			return Err(e.to_string());
		}
	};
}

fn main() {

	let args = Args::parse();

	if args.file.is_none() {
		print_error!("The following required argument was not provided:\n{}\n\nFor more information, try '--help'.", style("  <FILE>").green());
	}
	let file = args.file.unwrap();

	if args.verbose {
		print_info!("Reading from file.");
	}
	let code = match std::fs::read_to_string(&file) {
		Ok(o) => o,
		Err(e) => {
			print_error!("Could not open file: {}", e);
		}
	};
	if args.verbose {
		print_info!("Filtering characters from file.");
	}
	let code: String = code.lines().filter(|x| !x.starts_with("#")).collect();
	let code: String = code.chars().filter(|x| "><+-.,[]\n".contains(*x)).collect();

	let in_file = std::path::Path::new(&file).file_name().unwrap().to_str().unwrap();

	if args.verbose {
		print_info!("Testing for mismatched brackets.");
	}
	if code.matches("[").count() != code.matches("]").count() {
		print_error!("At {}: Mismatched brackets []", in_file);
	}

	if args.verbose {
		print_info!("Parsing code.");
	}
	let mut write_out: String = String::new();
	write_out.push_str("
#include <stdio.h>
#define red \"\\x1b[31m\"
#define reset \"\\x1b[0m\"
");

	write_out.push_str("int main() {");

	write_out.push_str(" long data[1000] = {0}; unsigned int current_index = 0; ");
	if args.debug {
		write_out.push_str("unsigned int biggest_index = 0; ");
	}

	let mut l = 1;
	let mut line_start = 0;
	for (i, c) in code.chars().enumerate() {
		match c {
			'+' => {
				write_out.push_str("data[current_index]++; ");
			},
			'-' => {
				write_out.push_str("data[current_index]--; ");
			},
			'>' => {
				write_out.push_str(&format!("if (current_index == 999) {{ printf(red \"[ERR] At {}:{}:{}\nCurrent index cannot be greater than 999\\n\" reset); return 1; }} ", in_file, l, i+1-line_start));
				write_out.push_str("current_index += 1; ");
				if args.debug {
					write_out.push_str("if (current_index > biggest_index) { biggest_index = current_index; } ");
				}
			},
			'<' => {
				write_out.push_str(&format!("if (current_index == 0) {{ printf(red \"[ERR] At {}:{}:{}\nCurrent index cannot be lower than 0\\n\" reset); return 1; }} ", in_file, l, i+1-line_start));
				write_out.push_str("current_index -= 1; ");
			},
			'.' => {
				// write_out.push_str(&format!("print!(\"{{}}\", String::from_utf8(vec![data[current_index] as u8]).unwrap_or_else(|e| {{ print_error!(\"At char {}: {{}}\", e); }} )); ", i+1));
				write_out.push_str("printf(\"%c\", (char)data[current_index]); ");
			},
			'[' => {
				write_out.push_str("while (data[current_index] != 0) { ");
			},
			']' => {
				write_out.push_str("} ");
			},
			',' => {
				write_out.push_str(&format!("int ret = scanf(\"%c\", &data[current_index]); if (ret < 0) {{ printf(red \"[ERR] At {}:{}:{}\nUnexpected EOF\\n\" reset); return 1; }} else if (ret == 0) {{ printf(red \"[ERR] At {}:{}:{}\nNo value assigned\\n\" reset); return 1; }} ", in_file, l, i+1-line_start, in_file, l, i+1-line_start));
			},
			'\n' => {
				l += 1;
				line_start = i+1;
			},
			_ => {}
		}
	}

	if code.contains('.') {
		write_out.push_str("printf(\"\\n\"); ");
	}

	if args.debug {
		write_out.push_str("printf(\"[\"); if (biggest_index > 0) { for (int i = 0; i < biggest_index; i++) { printf(\"%d, \", data[i]); } } printf(\"%d]\\n\", data[biggest_index]); ");
	}

	write_out.push_str("return 0; ");
	write_out.push('}');

	if args.verbose {
		print_info!("Getting tcc.");
	}
	let tcc = match tcc_path() {
		Ok(o) => o,
		Err(e) => {
			print_error!("Could not get path to compiler: {}", e);
		}
	};

	if args.verbose {
		print_info!("Compiling.");
	}
	let out = std::path::Path::new(&file).file_stem().unwrap().to_str().unwrap();
	let mut binding = std::process::Command::new(&tcc);
	let cmd = binding.arg("-Os").arg("-").arg("-o").arg(format!("{}.exe", out));
	if args.run {
		cmd.arg("-run");
	}

	match cmd.stdin(std::process::Stdio::piped()).spawn() {
		Ok(mut o) => {
			let mut stdin = o.stdin.as_mut().unwrap();
			let mut writer = std::io::BufWriter::new(&mut stdin);
			write!(writer, "{}", write_out).unwrap();
			drop(writer);
			o.wait().unwrap();
		},
		Err(e) => {
			print_error!("Could not call compiler: {}: {}", tcc.to_str().unwrap(), e);
		}
	}
	if args.verbose {
		print_info!("Done.");
	}

}