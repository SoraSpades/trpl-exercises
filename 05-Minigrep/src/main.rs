use minigrep::Config;

fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
