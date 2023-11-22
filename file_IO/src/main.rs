use std::io::BufWriter;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 4 {
        help_text("Too many arguments!");
        std::process::exit(1);
    } else if args.len() < 3 {
        help_text("Too few arguments!");
    }
    let filename = &args[2];
    if *(&args[1].eq("r")){
        println!{"{}", read_file(&filename)};
    } else if *(&args[1].eq("w")){
        let text_to_add = &args[3];
        println!{"The file was added -> {}", write_file(&filename, &text_to_add)};
    } else {
        help_text("Invalid arguments!");
    }
    
}

fn help_text(text: &str) {
    println!("{}", text);
    println!("Usage:");
    println!("- r filename (read file)");
    println!("- w filename text_to_add (write file)");
    std::process::exit(1);
}

fn read_file(filename: &str) -> String {
    let f = File::open(filename).expect("Unable to open file");
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).expect("Unable to read string");
    buffer
}

fn write_file(filename: &str, text: &str) -> String{
    let f = File::create(filename).expect("Unable to create file");
    {
        let mut writer = BufWriter::new(f);
        writer.write_all(text.as_bytes()).expect("Unable to write string");
    }
    return read_file(filename);
}
