use std::env;
use std::io;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let religion = env::var("RELIGION");

    if args.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        std::process::exit(0);
    }

    if args.iter().any(|a| a == "-v" || a == "--version") {
        println!("whoisgod {}", VERSION);
        std::process::exit(0);
    }

    if let Ok(religion) = religion
        && args.len() == 1
    {
        print_god(&religion);
        std::process::exit(0);
    }

    if args.len() != 3 || (args[1] != "-r" && args[1] != "--religion") {
        print_help();
        std::process::exit(2);
    }

    let religion = args.get(2).unwrap();

    print_god(religion);

    Ok(())
}

fn print_help() {
    eprintln!("Usage: whoisgod [OPTIONS]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -r, --religion <RELIGION>    Religion");
    eprintln!("  -h, --help                   Print this piece of text");
    eprintln!("  -v, --version                Print version");
}

fn islam() {
    println!("Allah");
}

fn christianity() {
    println!("God / The Holy Trinity: Father, Son, Holy Spirit");
}

fn print_god(religion: &str) {
    match religion.to_lowercase().as_str() {
        "islam" => islam(),
        "christianity" => christianity(),
        _ => print_help(),
    };
}
