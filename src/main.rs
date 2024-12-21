use std::{
    fs::File,
    io::{self, Read},
};
mod lex;
mod parser;

fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}


fn main() {

}
