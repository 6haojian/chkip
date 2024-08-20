use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
fn main() -> std::io::Result<()> {
    println!("========================");
    println!("                        ");
    println!("    IP CHECKING         ");
    println!("                        ");
    println!("========================");
    println!("                        ");
    println!("      LOADING...");
    println!("                        ");

    let mut file1 = String::new();
    let mut file2 = String::new();

    println!("Input the 1st file name:");
    io::stdin().read_line(&mut file1)?;

    println!("                        ");
    println!("Input the 2st file name:");
    io::stdin().read_line(&mut file2)?;

    let (file1, file2) = (file1.trim(), file2.trim());

    let f1 = File::open(file1)?;
    let f2 = File::open(file2)?;

    let f1_reader = io::BufReader::new(f1);
    let f2_reader = io::BufReader::new(f2);

    let mut f1_vec: Vec<String> = Vec::new();
    let mut f2_vec: Vec<String> = Vec::new();

    for line in f1_reader.lines() {
        let line = line?;
        f1_vec.push(line);
    }

    for line in f2_reader.lines() {
        let line = line?;
        f2_vec.push(line);
    }

    let f2_set: HashSet<_> = f2_vec.iter().cloned().collect();
    let f_diff: Vec<_> = f1_vec.iter().filter(|&x| !f2_set.contains(x)).collect();

    let mut diff: Vec<String> = f_diff.into_iter().map(|s| s.clone()).collect();

    f2_vec.append(&mut diff);

    let result = "/tmp/result.ip";

    let mut out_file = File::create(result)?;

    for i in &f2_vec {
        writeln!(out_file, "{}", i);
    }
    println!("                        ");
    println!(">>Saving file to {}     ", result);
    println!("                        ");

    println!("========================");
    println!("                        ");
    println!("       Finish           ");
    println!("                        ");
    println!("========================");

    Ok(())
}
