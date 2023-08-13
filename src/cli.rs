//
// Copyright 2023, [object Object]
// Licensed under MIT
//

use clap::{Arg, Command, ArgMatches};
use tui_tools::same_line_input;
fn get_args() -> ArgMatches {

    return Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("url")
                .required(true)
                .help("Url to download from."),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("type")
                .help("Type of request."),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .value_name("dry-run")
                .required(true)
                .help("Output file!"),
        )
        .arg(
            Arg::new("header")
                .short('H')
                .long("header")
                .value_name("header")
                .help("Add header to request."),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .num_args(0)
                .long("verbose")
                .help("Verbose output."),
        )
        .get_matches();
}

/// If the url is not supported, ask the user if they want to continue with the url
fn invalid_url(url: &String) {
    let msg = format!("The url {} is not supported. Do you want to continue? ", url);
    let input = same_line_input(msg.as_str());

    match input.trim().to_ascii_lowercase().as_str() {
        "y" | "yes" => {}
        _ => std::process::exit(0),
    }
}

fn validate_url(url: &String) {
    let url = url.as_str();
    let url = url.to_lowercase();

    if url.starts_with("http://") || url.starts_with("https://") {
        // DO NOTHING
        return;
    } else {
        invalid_url(&url);
    }
}

pub struct Cli {
    pub url: String,
    pub output: String,
    pub type_of_req: String,
    pub header: String,
    pub verbose: bool,
}

pub fn args() -> std::io::Result<Cli> {
    // Get the arguments
    let matches = get_args();

    // Get the url from the arguments
    let url = matches.get_one::<String>("url")
        .map(String::from)
        .unwrap_or_else(|| {
            eprintln!("No url provided.");
            std::process::exit(1);
        });

    // Validate the url
    validate_url(&url);

    let output = matches
        .get_one::<String>("output")
        .map(String::from)
        .unwrap_or_else(|| "output.txt".to_string());

    let type_of_req = matches
        .get_one::<String>("type")
        .map(String::from)
        .unwrap_or_else(|| "get".to_string());

    
    let header = matches
        .get_one::<String>("header")
        .map(String::from)
        .unwrap_or_else(|| "".to_string());

    let verbose = matches.get_flag("verbose");

    Ok(Cli {
        url,
        output,
        type_of_req,
        header,
        verbose,
    })
}