use anyhow::Result;

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
    let mut medium_num = 0;
    let mut large_num = 0;
    let mut found_triplet = false;
    let mut starting_index_to_delete_from = None;

    while !found_triplet {
        small_num = input[small_num_index];

        for (i, num) in input.iter().skip(small_num_index).enumerate() {
            // Since the number in the list after small_num will be greater than
            // or equal to small_num, multiply small_num by two so we can filter
            // out all the elements where the below sum is greater than 2020.
            let sum = 2 * small_num + num;

            if sum > 2020 {
                starting_index_to_delete_from = Some(i);
                break;
            }
        }

        if let Some(i) = starting_index_to_delete_from {
            input.truncate(i);
            starting_index_to_delete_from = None;
        }

        // Go through each medium number after the small_num, and just use Iterator::find
        // to find a medium and a large number that sums to 2020. Technically
        // suboptimal, but good enough for now.
        for (i, med_num) in input.iter().skip(small_num_index + 1).enumerate() {
            if let Some(lar_num) = input
                .iter()
                .skip(i + 1)
                .find(|n| small_num + med_num + **n == 2020)
            {
                medium_num = *med_num;
                large_num = *lar_num;
                found_triplet = true;
                break;
            }
        }

        small_num_index += 1;
    }

    println!("{}", small_num * medium_num * large_num);

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
