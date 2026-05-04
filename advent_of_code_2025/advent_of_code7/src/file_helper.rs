use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::manifold::Manifold;

pub fn load_file() -> Manifold{
    let file_string = fs::read_to_string("manifold.txt").unwrap();

    Manifold::new(file_string)
}
