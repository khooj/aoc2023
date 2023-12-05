use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn get_file_string() -> String {
    let args = env::args().collect::<Vec<String>>();
    let fp = args.get(1).unwrap();
    let mut f = File::open(fp).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}
