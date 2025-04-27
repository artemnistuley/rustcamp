use std::borrow::Cow;
use std::env;

// Example
// APP_CONF="/path/to/config.conf" cargo run
// cargo run -- --conf "/custom/path/to/config.conf"

fn main() {
    let config_path = get_config_path();

    println!("Config file path {}", config_path);
}

fn get_config_path() -> Cow<'static, str> {
    let args: Vec<String> = env::args().collect();

    for i in 0..args.len() - 1 {
        if args[i] == "--conf" {
            if args[i + 1].is_empty() {
                panic!("Error: --conf value cannot be empty");
            }
            return Cow::Owned(args[i + 1].clone());
        }
    }

    if let Ok(path) = env::var("APP_CONF") {
        if !path.is_empty() {
            return Cow::Owned(path);
        }
    }

    // Default path
    Cow::Borrowed("/etc/app/app.conf")
}
