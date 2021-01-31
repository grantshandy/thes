use clap::{crate_version, App, Arg};
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
                .required(false)
        )
        .get_matches();

        let word = match matches.value_of("WORD") {
            Some(data) => data,
            None => {
                eprintln!("ERROR: NO WORD FOUND! EXTREMELY UNEXPECTED!");
                "error"
            },
        };

        match synonym(word) {
            Ok(data) => {
                let mut synonyms = String::new();

                for synonym in data.synonyms.iter() {
                    if matches.is_present("vertical") {
                        synonyms.push_str(&format!("{}\n", synonym));
                    } else {
                        synonyms.push_str(&format!(" {}", synonym));
                    };
                };

                println!("{}", synonyms);
            },
            Err(error) => eprintln!("Error: {}", error),
        };
}
