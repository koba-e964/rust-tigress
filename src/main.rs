#![feature(plugin)]
#![cfg_attr(not(feature = "no-docopt-macros"),plugin(docopt_macros))]

extern crate calc;
extern crate rustc_serialize;
extern crate docopt;

use std::io;
use std::io::{Read,Write};
use std::fs::File;
use calc::parse;
use calc::interpret;
// use calc::typing;


// if feature = no-docopt-macros, this will not depend on docopt_macros.
#[cfg(not(feature = "no-docopt-macros"))]
docopt!(Args, "
Usage: calc-rust [options] [INPUT]

Options:
    -v, --verbose  Verbose mode
    -t, --typing   Check types
");
#[cfg(feature = "no-docopt-macros")]
#[allow(non_snake_case)]
struct Args {
    flag_verbose: bool,
    arg_INPUT: String, // needs allow(non_snake_case) because of this line
}

#[cfg(not(feature = "no-docopt-macros"))]
fn get_args() -> Args {
    Args::docopt()
        .decode()
        .unwrap_or_else(|e| e.exit())
}
#[cfg(feature = "no-docopt-macros")]
fn get_args() -> Args {
    Args { flag_verbose: false, arg_INPUT: "".to_string() }
}

fn main() {
    let args: Args = get_args();
    if args.flag_verbose {
        println!("verbose mode");
    }
    let mut s: String = "".to_string();
    if args.arg_INPUT == "".to_string() { // Reads from stdin
        print!("> ");
        io::stdout().flush().ok().unwrap();
        match io::stdin().read_line(&mut s) {
            Ok(_) => {}
            Err(err) => { panic!(err); }
        }
    } else { // Reads from file
        let mut fp = File::open(args.arg_INPUT)
            .unwrap_or_else(|e| panic!(e));
        fp.read_to_string(&mut s)
            .unwrap_or_else(|e| panic!(e));
    }
    let ast = parse::parse(&s);
    if args.flag_verbose {
        println!("{:?}", ast);
    }
    println!("result = {}", interpret::f(&ast, args.flag_verbose));
}
