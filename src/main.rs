use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

// print version
fn print_version(file: &FileConfig) {
    println!("Version of program : {}", file.version);
    exit(1);
}

// print file data
fn cat(file: &mut dyn BufRead) {
    for line in file.lines() {
        println!("{}", line.unwrap());
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

    // implementing default configurations
    let mut file_conf: FileConfig = FileConfig::default();

    // WARN: clone is not best practise
    // TODO: replace clone in future
    // last argument is filepath
    let file_name: Option<String> = args.last().cloned();

    // WARN: clone is not best practise
    // TODO: replace clone in future
    file_conf.file = file_name.clone();

    // help
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        // TODO: make a help guide
        exit(1);
    }

    // version
    if args.contains(&"-v".to_string()) || args.contains(&"--Version".to_string()) {
        print_version(&file_conf);
        exit(1);
    }

    // error handling
    match File::open(file_name.unwrap()) {
        Ok(file) => {
            let mut input = BufReader::new(file);
            //TODO: Check read permissions
            cat(&mut input);
        }
        Err(_) => {
            println!("No such file or directory.");
        }
    }
}
