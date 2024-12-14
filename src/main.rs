use std::{
    fs::File,
    io::{self, Read},
};

mod lex;

fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn process_escape_sequences(input: &mut String) -> String {
    //#[logos(subpattern escaped_char = r#"[ \n\r\t\v\a\f\\\'\x08]"# )]
    input
        .replace(r"\\", "\\")    
        .replace(r"\'", "\'")    
        .replace(r"\n", "\n")    
        .replace(r"\r", "\r")    
        .replace(r"\x08", "\x08")
        .replace(r"\v", "\x0b")   
        .replace(r"\f", "\x0c")  
        .replace(r"\a", "\x07")  
        .replace(r"\t", "\t")    
        .replace(r#"\""#, "\"")    
}

fn main() {
    let mut input_f =
        read_file_to_string("src/lex/testcase/rune.txt").expect("Failed to read file");

    for pat in input_f.split("\n") {
        println!("{}", pat)
    }
}
