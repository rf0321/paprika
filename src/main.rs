extern crate paprika;
use paprika::window;
use paprika::html;
use std::default::Default;
use std::io::{Read, BufWriter};
use std::fs::File;

fn main(){
    let html = read_source(String::from("/home/username/paprika-browser/examples/index.html"));
    println!("{:?}", html::parse(html));    
}
fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}
