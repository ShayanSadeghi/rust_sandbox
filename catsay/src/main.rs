extern crate structopt; // use "StructOpt" crate to work with arguments in a simpler way and making help, version flags automatically

use structopt::StructOpt;

#[derive(StructOpt)] //annotate the struct with this to StructOpt takes this struct as the argument definition
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appera dead
    dead: bool,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    let eye = if options.dead { "x" } else { "o" };

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("       /\\_/\\");
    println!("     ( {eye}   {eye} )", eye = eye);
    println!("      =( I )=");
}

// cargo run -- "Hello world!"
