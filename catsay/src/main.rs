extern crate structopt; // use "StructOpt" crate to work with arguments in a simpler way and making help, version flags automatically

use structopt::StructOpt;

#[derive(StructOpt)] //annotate the struct with this to StructOpt takes this struct as the argument definition
struct Options {
    message: String,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("      /\\_/\\");
    println!("     ( o  o )");
    println!("      =( )=");
}

// cargo run -- "Hello world!"
