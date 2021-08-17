use std::{collections::HashMap, io::{BufRead, BufReader, Read}, path::PathBuf, sync::atomic::AtomicUsize, time::{Duration, Instant}};

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

    let mut bytes = AtomicUsize::new(0);

    let ticker = std::thread::spawn(move ||{
        let start_f = Instant::now();

        loop {
            std::thread::sleep(Duration::from_secs(1));

            let total_bytes = bytes.load(std::sync::atomic::Ordering::Relaxed);
            
            if total_bytes == 0 {
                return;
            }

            let elapsed = start_f.elapsed();
            let sec: f64 = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0);
            let rate = (total_bytes as f64 / sec) as usize;
            eprintln!("bytes: rate {}/s  total {}                          \r", mem_metric_digit(rate,4), mem_metric_digit(total_bytes,4));
        }
    });

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


fn mem_metric<'a>(v: usize) -> (f64, &'a str) {
    const METRIC: [&str; 8] = ["B ", "KB", "MB", "GB", "TB", "PB", "EB", "ZB"];

    let mut size = 1usize << 10;
    for e in &METRIC {
        if v < size {
            return ((v as f64 / (size >> 10) as f64) as f64, e);
        }
        size <<= 10;
    }
    (v as f64, "")
}

/// keep only a few significant digits of a simple float value
fn sig_dig(v: f64, digits: usize) -> String {
    let x = format!("{}", v);
    let mut d = String::new();
    let mut count = 0;
    let mut found_pt = false;
    for c in x.chars() {
        if c != '.' {
            count += 1;
        } else {
            if count >= digits {
                break;
            }
            found_pt = true;
        }

        d.push(c);

        if count >= digits && found_pt {
            break;
        }
    }
    d
}

pub fn mem_metric_digit(v: usize, sig: usize) -> String {
    if v == 0 || v > std::usize::MAX / 2 {
        return format!("{:>width$}", "unknown", width = sig + 3);
    }
    let vt = mem_metric(v);
    format!("{:>width$} {}", sig_dig(vt.0, sig), vt.1, width = sig + 1, )
}

