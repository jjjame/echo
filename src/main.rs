use clap::{Arg, Command};
fn main() {
    let matches = Command::new("Echo")
        .version("0.1")
        .author("jjjame")
        .about("rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("The text to echo")
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0),
        )
        .get_matches();

    let text: &String = matches.get_one("text").unwrap();
    let omit_newline: bool = matches.get_flag("omit_newline");

    println!("text {}, omit_newline {}", text, omit_newline);
}
