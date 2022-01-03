#![allow(unused)]
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

fn main() {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    find_matches(&content, &args.pattern, &mut std::io::stdout());
}

#[test]
fn test_find_matches() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "ipsum", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
