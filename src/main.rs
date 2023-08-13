//
// Copyright 2023, [object Object]
// Licensed under MIT
//

use std::{fs::File, io::{Read, Write, self}};
use reqwest::{blocking::{RequestBuilder, Client}, StatusCode};
use tui_tools::Colors;
mod cli;

fn build_req(client: &Client, url: &String, type_of_req: &str) -> reqwest::blocking::RequestBuilder {
    // requestBuilder based on the type of request.
    let build_client: RequestBuilder = match type_of_req {
        "get" | "GET" => {
            client.get(url)
        }, 
        "post" | "POST" => {
            client.post(url)
        },
        "put" | "PUT" => {
            client.put(url)
        },
        "delete" | "DELETE" => {
            client.delete(url)
        },
        "head" | "HEAD" => {
            client.head(url)
        },
        _ => {
            println!("{:?}", type_of_req);
            println!("Invalid type of request!");
            std::process::exit(1);
        }
    };

    return build_client;
}

fn split_header(header: &str) -> (String, String) {
    // Split the header into a vector of strings.
    // Header should be in the format of "key: value" ex: "Content-Type: application/json"
    let header_key = header.split(":").collect::<Vec<&str>>()[0].replace('"', "");
    let header_value = header.split(":").collect::<Vec<&str>>()[1].replace('"', "");

    return (header_key, header_value);
}

fn print_downloading_bar(percent: u32) {
    let bars = percent / 5;

    // Print a downloading bar "Downloading: [====================]"
    print!("\r{}", "Downloading: [".bright_green());
    for i in 0..20 {
        if i < bars {
            print!("{}", "=".bright_green());
        } else {
            print!(" ");
        }
    }

    let percent = format!("{:3}", percent);

    // Ads the percent to the bar "Downloading: [====================] 100%"
    print!("{}{}{}", "] ".bright_green(), percent.as_str().bright_green(), "%".bright_green());
}

fn download(mut response: reqwest::blocking::Response, output: &String) -> io::Result<()> {
    let mut file = File::create(output)?;

    let total_bytes = response.content_length().unwrap_or(0);
    let mut bytes_written = 0;

    // create a loading bar
    print_downloading_bar(0);

    // Create a buffer to store the response in.
    let mut buffer = [0; 1024];
    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }
        // Write the buffer to the file.
        file.write_all(&buffer[..n])?;

        // Keep track of how many bytes have been written for the progress bar.
        bytes_written += n as u64;

        // Calculate the percent of the file that has been downloaded.
        let percent = (bytes_written as f64 / total_bytes as f64 * 100.0) as u32;

        print_downloading_bar(percent);
    }
    
    println!("\n{}", "Downloaded!".bold_bright_blue());

    Ok(())
}

fn main() -> io::Result<()> {
    // Get args
    let args = cli::args()?;

    if args.verbose { println!("Got args!")}

    // Create the blocking client.
    let client = Client::new();

    if args.verbose { println!("Created Client!")}

    // Build the requestbuilder.
    let built_req = build_req(&client, &args.url, &args.type_of_req);

    // Send the request and unwrap the response.
    // If header is not empty, then add it to the request.
    let response = if !args.header.is_empty() {
        // Split the header into a tuple of (key, value)
        let header = split_header(&args.header);

        if args.verbose { println!("Created Header! ({}: {})", header.0, header.1)};

        if args.verbose { println!("Sending req!")}
        built_req.header(header.0, header.1).send().unwrap_or_else(|e| {
            println!("Failed to send request! {e}");
            std::process::exit(1);
        })

    } else {
        if args.verbose { println!("Sending req!")}

        // If the header is empty, then just send the request.
        built_req.send().unwrap_or_else(|e| {
            println!("Failed to send request! {e}");
            std::process::exit(1);
        })
    };

    // If the response is not 200 OK, then exit the program.
    if response.status() != StatusCode::OK {
        println!("Request failed with status code: {}", response.status());
    }

    if args.verbose { println!("Recieved response with {}!", response.status())}

    // Start downloading the file.
    download(response, &args.output).unwrap_or_else(|err| {
        println!("Failed to download file: {err}");
        std::process::exit(1);
    });

    if args.verbose { println!("Downloaded file!")}

    Ok(())
}
