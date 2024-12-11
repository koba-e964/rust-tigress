use calc::interpret;
use calc::parse;
use clap::{Arg, Command};
use std::fs::File;
use std::io;
use std::io::{Read, Write};

#[allow(non_snake_case)]
struct Args {
    flag_verbose: bool,
    arg_INPUT: String, // needs allow(non_snake_case) because of this line
}

fn get_args() -> Args {
    let matches = Command::new("rust-tigress")
        .version("0.1.0")
        .author("koba-e964 <3303362+koba-e964@users.noreply.github.com>")
        .about("Simple calculator")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Verbose output"),
        )
        .arg(
            Arg::new("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .index(1),
        )
        .get_matches();
    Args {
        flag_verbose: matches.contains_id("verbose"),
        arg_INPUT: matches
            .get_one::<String>("INPUT")
            .unwrap_or(&"".to_string())
            .to_string(),
    }
}
fn main() {
    let args: Args = get_args();
    if args.flag_verbose {
        println!("verbose mode");
    }
    let mut s: String = "".to_string();
    if args.arg_INPUT.is_empty() {
        // Reads from stdin
        print!("> ");
        io::stdout().flush().ok().unwrap();
        match io::stdin().read_line(&mut s) {
            Ok(_) => {}
            Err(err) => {
                panic!("{err}");
            }
        }
    } else {
        // Reads from file
        let mut fp = File::open(args.arg_INPUT).unwrap_or_else(|e| panic!("{e}"));
        fp.read_to_string(&mut s).unwrap_or_else(|e| panic!("{e}"));
    }
    let ast = parse::parse(&s);
    if args.flag_verbose {
        println!("{:?}", ast);
    }
    println!("result = {}", interpret::f(&ast, args.flag_verbose));
}
