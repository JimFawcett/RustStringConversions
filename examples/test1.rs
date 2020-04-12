extern crate rust_string_conversions;

use std::io;
#[allow(unused_imports)]
use rust_string_conversions::{demo1, demo2};

fn main() -> io::Result<()> {

    demo1()?;
    //demo2()?;

    println!("\n\n  That's all Folks!\n");
    Ok(())
}
