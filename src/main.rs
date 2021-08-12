use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

use anyhow::Result;

mod decomp;

fn main() {
    match run() {
        Err(e) => println!("error: {}", e),
        _ => {}
    }
}

fn run() -> Result<()> {
    // println!("Hello, world!");
    let args:Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: charcounts <character> file1 [file2 .. fileN]");
        std::process::exit(1);
    }
    let to_find = args.get(1).unwrap().as_bytes()[0];
    let mut buf = [0; 64*4096];
    let mut hm: HashMap<u64, u64> = HashMap::new();
    let mut line_count = 0;
    let mut curr_char = 0;
    for f in args.iter().skip(2) {
        let path = PathBuf::from(f);
        // println!("reading file {}", &path.display());
        let mut rdr = BufReader::new(crate::decomp::open_comp_file(&path)?);

        loop {
        let len = rdr.read(&mut buf[..])?;
        if len == 0 {
            break;
        }

        buf[0..len].iter().for_each(|b|{
            if *b == to_find {
                curr_char += 1;
            } else if *b == b'\n' || *b == b'\r' {
                line_count += 1;
                if let Some(x) = hm.get_mut(&curr_char) {
                    *x += 1;
                } else {
                    hm.insert(curr_char, 1);
                }               
                curr_char = 0;
            }
        });
        }
    }

    let mut cc_vec: Vec<(u64,u64)> = hm.iter().map(|(k,v)| (*k,*v)).collect();
    cc_vec.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    for (k, v) in cc_vec {
        println!("{}  at  {}", k, v);
    }
    println!("line count: {}", line_count);
    Ok(())
}
