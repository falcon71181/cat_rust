use std::env;
use std::process::exit;

// File Structure along with all arguments
struct FileConfig {
    // store file location + file name
    file: Option<String>,

    // TODO : show line numbers along side
    number: bool,

    // Version
    version: String,
}

// Default trait for file FileConfig
impl Default for FileConfig {
    fn default() -> FileConfig {
        FileConfig {
            file: None,
            number: false,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

fn main() {
    // grab all the arguments from the command
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // TODO : make list selector of files present alike ranger
        println!("Usage: Provide me file name");
        args.push("--help".to_string());
        // Run the command here and exit
        exit(1);
    }

    println!("{:?}", args);
}
