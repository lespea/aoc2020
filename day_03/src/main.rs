use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Error, Result};
use atoi::atoi;
use bstr::io::BufReadExt;

const FIRST: bool = false;
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
        let num = atoi::<usize>(line).context("invalid num found").unwrap();
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

fn get_file<P>(path: P) -> Result<HashSet<usize>>
where
    P: AsRef<Path>,
{
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut nums = HashSet::with_capacity(10_000);
    reader.for_byte_line(|line| {
        let num = atoi::<usize>(line).context("invalid num found").unwrap();
        if num < TARGET {
            nums.insert(num);
        }
        Ok(true)
    })?;

    Ok(nums)
}

fn proc_nums(hs: &HashSet<usize>, new_target: Option<usize>) -> Option<usize> {
    match new_target {
        Some(target) => {
            for num in hs.iter() {
                if target > *num {
                    let nt = target - num;
                    if hs.contains(&nt) {
                        let orig = TARGET - target;
                        dbg!(orig, num, nt);
                        return Some(orig * num * nt);
                    }
                }
            }
        }

        None => {
            for num in hs.iter() {
                let r = proc_nums(hs, Some(TARGET - num));
                if r.is_some() {
                    return r;
                }
            }
        }
    }

    None
}

fn main() -> Result<()> {
    if FIRST {
        p_file("input1")?;
    } else {
        println!(
            "{}",
            proc_nums(&get_file("input1")?, None).context("didn't find triple num")?
        );
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use crate::proc_nums;

    #[test]
    fn example() {
        assert_eq!(
            proc_nums(
                &HashSet::from_iter(vec![1721, 979, 366, 299, 675, 1456]),
                None,
            ),
            Some(241861950),
        )
    }
}
