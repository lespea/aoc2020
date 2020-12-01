use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Error, Result};
use atoi::atoi;
use bstr::io::BufReadExt;
use std::collections::HashSet;

const TARGET: usize = 2020;

fn p_file<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut nums = HashSet::with_capacity(10_000);
    let mut found = false;

    reader.for_byte_line(|line| {
        let num = atoi::<usize>(line).expect("bad num");
        if num < TARGET {
            let diff = TARGET - num;
            if nums.contains(&diff) {
                println!("{}", num * diff);
                found = true;
                Ok(false)
            } else {
                nums.insert(num);
                Ok(true)
            }
        } else {
            Ok(true)
        }
    })?;

    if found {
        Ok(())
    } else {
        Err(Error::msg("sum not found"))
    }
}

fn main() -> Result<()> {
    p_file("input1")?;
    Ok(())
}

#[cfg(test)]
mod test {}
