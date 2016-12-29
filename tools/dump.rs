extern crate javabc;

use javabc::raw::Serializable;

use std::{fs, env, io};
use std::io::prelude::*;

fn read(file_path: &str)
    -> Result<(), io::Error> {
    let mut file = fs::File::open(file_path)?;

    let class_file = javabc::raw::ClassFile::read(&mut file)?;
    println!("{:#?}", class_file);

    Ok(())
}

fn main() {
    if let Some(file_path) = env::args().skip(1).next() {
        if let Err(e) = read(&file_path) {
            println!("error: {}", e);
        }
    } else {
        println!("please enter an input file");
    }
}
