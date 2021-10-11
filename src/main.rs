use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};
use colored::Colorize;
use std::io;
use thesaurus::{Thesaurus, WordType};

fn main() {
    // Set CLI application details through clap.
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("WORD")
                .help("Word to find synonyms for")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("type")
                .long("type")
                .short("t")
                .value_name("PART OF SPEECH")
                .help("Select what parts of speech the synonyms returned will have")
                .possible_values(&["verb", "adjective", "adverb", "noun"])
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Prints verbose output, this includes parts of speech for each word")
                .required(false),
        )
        .arg(
            Arg::with_name("horizontal")
                .long("horizontal")
                .help("Prints synonyms horizontally")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("shell")
                .long("shell")
                .short("s")
                .help("Opens an interactive thesaurus shell")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    let word = get_word(matches.clone());

    let word_type: Option<WordType> = matches
        .value_of("type")
        .map(|word_type| full_name_to_word_type(word_type));

    thesaurus(matches, word, word_type);
}

fn thesaurus(app: ArgMatches, word: String, word_type: Option<WordType>) {
    match Thesaurus::synonym(word.to_lowercase(), word_type) {
        Ok(data) => {
            let mut synonyms = String::new();

            for synonym in data.words.iter() {
                if synonym.name == word.to_lowercase() {
                    continue;
                };

                if app.is_present("horizontal") {
                    if app.is_present("verbose") {
                        synonyms.push_str(&format!("{} ({}), ", synonym.name, synonym.word_type));
                    } else {
                        synonyms.push_str(&format!("{} ", synonym.name));
                    };
                } else if app.is_present("verbose") {
                    synonyms.push_str(&format!("{} ({})\n", synonym.name, synonym.word_type));
                } else {
                    synonyms.push_str(&format!("{}\n", synonym.name));
                };
            }

            synonyms = (&synonyms[0..synonyms.len() - 1]).to_string();

            println!("{}", synonyms);
        }
        Err(error) => {
            clap_error(error.to_string().as_str());
        }
    };
}

// Designed to imitate clap's errors so we can have our own errors output that look exactly the same :)
fn clap_error(error: &str) {
    eprintln!("{} {}\n", "error:".red().bold(), error);
    eprintln!("USAGE:\n    thes [FLAGS] [OPTIONS] [WORD]\n");
    eprintln!("For more information try {}", "--help".green());
}

fn full_name_to_word_type(word_type: &str) -> WordType {
    match word_type {
        "verb" => WordType::Verb,
        "adjective" => WordType::Adjective,
        "adverb" => WordType::Adverb,
        "noun" => WordType::Noun,
        &_ => {
            clap_error(&format!("{} had an unexpected input, but was still let through. This is extremely unexpected.", "<TYPE>".yellow()));
            std::process::exit(1);
        }
    }
}

fn get_stdin() -> Option<String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(len) => {
            let len = len - 1;

            if len == 0 {
                None
            } else {
                Some(input)
            }
        }
        Err(error) => {
            clap_error(&format!("Error getting Stdin: {}", error));
            std::process::exit(1);
        }
    }
}

fn get_word(app: ArgMatches) -> String {
    return match app.value_of("WORD") {
        Some(data) => data.to_string(),
        None => match get_stdin() {
            Some(data) => return (&data[0..data.len() - 1]).to_string(),
            None => {
                clap_error(&format!(
                    "The following required arguments were not provided:\n    {}",
                    "<WORD>".red().bold()
                ));
                std::process::exit(1);
            }
        },
    };
}
