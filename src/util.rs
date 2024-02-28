use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> std::io::Result<impl Iterator<Item = std::io::Result<String>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}
