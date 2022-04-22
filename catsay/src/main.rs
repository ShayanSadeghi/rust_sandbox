extern crate colored;
extern crate exitfailure;
extern crate failure;
extern crate structopt; // use "StructOpt" crate to work with arguments in a simpler way and making help, version flags automatically

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(StructOpt)] //annotate the struct with this to StructOpt takes this struct as the argument definition
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appera dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>, // use "Option" to make this optional
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let message = options.message;
    let eye = if options.dead { "x" } else { "o" };

    // make an STDERR by 'eprintln'.
    // use 2> and 1> to write STDERR and STDOUT into separate files
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog!")
    }

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;

            let mut cat_picture = cat_template.replace("{eye}", eye);
            cat_picture = cat_picture.replace("{message}", &message);
            println!("{}", &cat_picture);
        }
        None => {
            // 'println' makes STDOUT
            println!("{}", message.bright_yellow().on_blue().underline());
            println!(" \\");
            println!("  \\");
            println!("       /\\_/\\");
            println!("     ( {eye}   {eye} )", eye = eye.bold().red());
            println!("      =( I )=");
        }
    }
    Ok(())
}

// cargo run -- "Hello world!"
