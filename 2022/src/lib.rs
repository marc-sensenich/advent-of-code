use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// read_lines from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file_to_string<P>(filename: P) -> Result<String, std::io::Error>
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(filename)
}

// pub fn read_file_to_string(filename: P) -> Result<std::string::String>
// where
//     P: AsRef<Path>,
// {
//     Ok(std::fs::read_to_string(filename))
// }
