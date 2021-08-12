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
    println!("Hello, world!");
    let to_find = b'.';
    let mut buf = vec![0; 4096];
    let mut hm: HashMap<u64, u64> = HashMap::new();
    let mut line_count = 0;
    let mut curr_char = 0;
    for f in std::env::args().skip(1) {
        let path = PathBuf::from(f);
        println!("reading file {}", &path.display());
        let mut rdr = BufReader::new(crate::decomp::open_comp_file(&path)?);

        let len = rdr.read(&mut buf[..])?;

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

        // for i in 0..len {
        //     let b = buf[i];
        //     if b == to_find {
        //         curr_char += 1;
        //     } else if b == b'\n' || b == b'\r' {
        //         line_count += 1;
        //         if let Some(x) = hm.get_mut(&curr_char) {
        //             *x += 1;
        //         } else {
        //             hm.insert(curr_char, 1);
        //         }               
        //         curr_char = 0;
        //     }
        // }
    }

    //let cc_vec = hm.iter().map(|(k,v| (k,v)).collect::Vec<u64,u64>();
    for (k, v) in hm {
        println!("{}  at  {}", k, v);
    }
    println!("line count: {}", line_count);
    Ok(())
}
