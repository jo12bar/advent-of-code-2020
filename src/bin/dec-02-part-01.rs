use anyhow::Result;
use regex::Regex;

const INPUT_FILE: &str = "./input/dec-02-part-01/input.txt";

fn main() -> Result<()> {
    let re = Regex::new(r"^(?P<lo>\d+)-(?P<hi>\d+)\s*(?P<policy>.):(?P<pswd>.+)")?;

    let mut num_valid_passwords = 0_usize;

    for line in single_buffer_reader::SingleBufferReader::open(INPUT_FILE)? {
        let line = line?;

        if let Some(caps) = re.captures(line.as_ref()) {
            let low: usize = caps["lo"].parse()?;
            let high: usize = caps["hi"].parse()?;
            let policy = caps["policy"].chars().next().unwrap();
            let password = caps["pswd"].trim();

            let num_policy_chars = password.chars().filter(|c| c == &policy).count();

            if num_policy_chars >= low && num_policy_chars <= high {
                num_valid_passwords += 1;
            }
        }
    }

    println!("Number of valid passwords: {}", num_valid_passwords);

    Ok(())
}

mod single_buffer_reader {
    #![allow(clippy::rc_buffer)]

    use std::{
        fs::File,
        io::{self, prelude::*},
        rc::Rc,
    };

    pub struct SingleBufferReader {
        reader: io::BufReader<File>,
        buf: Rc<String>,
    }

    const BUF_SIZE: usize = 1024; // bytes

    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(BUF_SIZE))
    }

    impl SingleBufferReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { reader, buf })
        }
    }

    impl Iterator for SingleBufferReader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let buf = match Rc::get_mut(&mut self.buf) {
                Some(buf) => {
                    buf.clear();
                    buf
                }
                None => {
                    self.buf = new_buf();
                    Rc::make_mut(&mut self.buf)
                }
            };

            self.reader
                .read_line(buf)
                .map(|u| {
                    if u == 0 {
                        None
                    } else {
                        Some(Rc::clone(&self.buf))
                    }
                })
                .transpose()
        }
    }
}
