use clap::{crate_version, App, Arg};
use colored::Colorize;
use thesaurus::synonym;

fn main() {
    // Set CLI application details through clap.
    let matches = App::new("thes")
        .version(crate_version!())
        .author("Grant H. <grantshandy@gmail.com>")
        .about("Finds synonyms")
        .arg(
            Arg::with_name("WORD")
                .help("Word to find synonyms for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("vertical")
                .long("vertical")
                .short("v")
                .help("Prints synonyms vertically")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    let word = match matches.value_of("WORD") {
        Some(data) => data,
        None => {
            eprintln!("{} {} wasn't detected by clap, but was still let through. This is extremely unexpected.\n", "error:".red().bold(), "<WORD>".yellow());
            eprintln!("USAGE:\n    thes [FLAGS] <WORD>");
            eprintln!("\nFor more information try {}", "--help".green());
            "error"
        }
    };

    match synonym(word) {
        Ok(data) => {
            let mut synonyms = String::new();

            for synonym in data.synonyms.iter() {
                if synonym == word {
                    continue;
                };

                if matches.is_present("vertical") {
                    synonyms.push_str(&format!("{}\n", synonym));
                } else {
                    synonyms.push_str(&format!("{} ", synonym));
                };
            }

            synonyms = (&synonyms[0..synonyms.len() - 1]).to_string();

            println!("{}", synonyms);
        }
        Err(error) => {
            eprintln!("{} {}\n", "error:".red().bold(), error);
            eprintln!("USAGE:\n    thes [FLAGS] <WORD>");
            eprintln!("\nFor more information try {}", "--help".green());
        }
    };
}
