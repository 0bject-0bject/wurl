# WURL

## What is WURL?
Wurl is a tool that allows you to download files from the web using a simple command line interface. It is written in Rust and uses the [reqwest](https://docs.rs/reqwest/latest/reqwest/) crate to make HTTP requests.

## How do I use WURL?
Wurl is very simple to use. Just type `wurl --url <url> -o C:\Users\user\Downloads\<file.extension>` and it will download the file to the current directory. For more information, type `wurl --help`.

## How do I install WURL?

1. Clone the repository
2. Run `cargo build --release`
3. Copy the executable from `target/release/wurl.exe` to a directory in your PATH

### OR!!

1. cargo install wurl

(thats it!!)

## What is the license for WURL?
Wurl is licensed under the MIT license. See the [LICENSE](https://github.com/0bject-0bject/wurl/blob/42cc6a4871c378270d49b37790c0970de31326f2/LICENSE) file for more information.

## What is the future of WURL?
Wurl is currently in development. I plan to add more features to it in the future, such as the ability to download multiple files at once, and more compatability. If you have any suggestions, feel free to open an issue or a pull request.
