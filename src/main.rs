// user custom codes
mod gen_mama_str;
mod help;
mod word_part;

// crates
extern crate clap;

// specifically
use std::panic::panic_any;
use clap::{App, Arg};

// macros
pub const system_name: &str = "digimama";


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parser for command line arguments
    let app = App::new(system_name)
        .version("1.0")
        .author("jumango")
        .arg(Arg::with_name("help")
                 .help("show help")
                 .short("h")
                 .long("help")
        );

    let matches = app.get_matches();

    // help eval
    if matches.is_present("help") {
        for line in help::gen_help_text() {
            println!("{}", line);
        }
        // exit
        std::process::exit(0);
    }

    // generate mama str
    let mama_str = gen_mama_str::gen_mama_str();
    println!("{}", mama_str);

    Ok(())
}
