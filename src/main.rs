use clap::{Arg, ArgAction, Command};
fn main() {
    let matches = Command::new("echor")
        .version("0.1")
        .author("jjjame")
        .about("rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("The text to echo")
                .action(ArgAction::Append)
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0),
        )
        .get_matches();

    let text: Vec<&str> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    let omit_newline: bool = matches.get_flag("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}
