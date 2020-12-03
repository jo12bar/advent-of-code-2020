use anyhow::Result;
use std::cmp::Ordering;

const INPUT_PATH: &str = "./input/dec-01-part-01/input.txt";

fn main() -> Result<()> {
    let mut input: Vec<u32> = Vec::new();

    for line in single_buffer_reader::SingleBufferReader::open(INPUT_PATH)? {
        let number: u32 = line?.trim().parse()?;
        input.push(number);
    }

    // Sort the input:
    input.sort_unstable();

    let mut small_num_index = 0;
    let mut small_num = 0;
    let mut large_num = 0;
    let mut found_pair = false;
    let mut starting_index_to_delete_from = None;

    while !found_pair {
        small_num = input[small_num_index];

        for (i, num) in input.iter().skip(small_num_index).enumerate() {
            let sum = small_num + num;

            match sum.cmp(&2020) {
                Ordering::Greater => {
                    starting_index_to_delete_from = Some(i);
                    break;
                }
                Ordering::Equal => {
                    large_num = *num;
                    found_pair = true;
                    break;
                }
                _ => {}
            }
        }

        if let Some(i) = starting_index_to_delete_from {
            input.truncate(i);
            starting_index_to_delete_from = None;
        }

        small_num_index += 1;
    }

    println!("{}", small_num * large_num);

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
