use std::env;

// File Structure along with all arguments
struct FileConfig {
    // store file location + file name
    file: String,

    // TODO : show line numbers along side
    number: bool,

    // Version
    version: String,
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // TODO : make list selector of files present alike ranger
        args.push("--help".to_string());
    }

    println!("{:?}", args);
}
