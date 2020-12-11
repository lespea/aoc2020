use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Error, Result};
use atoi::FromRadix10;
use bstr::io::BufReadExt;

#[derive(Eq, PartialEq, Debug)]
struct Info<'a> {
    low: usize,
    high: usize,
    c: u8,
    str: &'a [u8],
}

fn from_line<'b, 'a: 'b>(line: &'a [u8]) -> Result<Info<'b>> {
    let (low, hpos) = usize::from_radix_10(line);
    if hpos == 0 {
        return Err(Error::msg("invalid low"));
    }
    let (high, lpos) = usize::from_radix_10(&line[hpos + 1..]);
    if lpos == 0 {
        return Err(Error::msg("invalid high"));
    }

    let cstart = hpos + lpos + 2;

    let c = line[cstart];
    let str = &line[cstart + 3..];

    Ok(Info {
        low,
        high,
        c,
        str,
    })
}

fn p_file<P, F>(path: P, c: &mut usize, fun: F) -> Result<()>
    where
        P: AsRef<Path>,
        F: Fn(Info<'_>, &mut usize) -> (),
{
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    // let mut nums = HashSet::with_capacity(10_000);
    reader.for_byte_line(|line| {
        fun(from_line(line).unwrap(), c);
        Ok(true)
    })?;

    Ok(())
}

fn main() -> Result<()> {
    let mut valid = 0;

    p_file(
        "input",
        &mut valid,
        |info, count| {
            let cnt = info.str.iter().filter(|&b| *b == info.c).count();
            if cnt >= info.low && cnt <= info.high {
                *count += 1;
            }
        },
    )?;
    println!("{}", valid);

    valid = 0;
    p_file(
        "input",
        &mut valid,
        |info, count| {
            if (info.str[info.low - 1] == info.c) ^ (info.str[info.high - 1] == info.c) {
                *count += 1;
            }
        },
    )?;
    println!("{}", valid);

    Ok(())
}


#[cfg(test)]
mod test {
    use crate::from_line;

    #[test]
    fn example() {
        assert_eq!(
            from_line(b"1-3 a: abcde").unwrap(),
            crate::Info { low: 1, high: 3, c: b'a', str: b"abcde" }
        );
        assert_eq!(
            from_line(b"1-3 b: cdefg").unwrap(),
            crate::Info { low: 1, high: 3, c: b'b', str: b"cdefg" }
        );
        assert_eq!(
            from_line(b"22-9 c: cccccccccc").unwrap(),
            crate::Info { low: 22, high: 9, c: b'c', str: b"cccccccccc" }
        );
    }
}
