use clap::Parser;
use args::TodoArgs;
use file::get_file;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;




mod args;
mod add; 
mod file;


pub fn main() {
    let args = TodoArgs::parse();
    let path = args.path;
    let path_str = get_file(path.clone());    
}
