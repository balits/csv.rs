use std::{fs, io::Read};
fn main() {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open("input/random.csv")
        .unwrap();

    let mut s = String::default();
    file.read_to_string(&mut s).unwrap();

    read_csv(&s);
}

fn read_csv(input: &str) {
    let header = sniff::has_header(input);

    let keys = if header {
        sniff::get_header_keys(input);
    } else {
        let column_hint = sniff::column_hint(inp)
    }
}

mod sniff {
    pub fn has_header(input: &str) -> bool {
        let mut first_line = None;
        for (counter, line) in input.lines().enumerate() {
            if counter == 20 {
                break;
            }

            if line.starts_with("#") {
                continue;
            }

            if first_line.is_none() {
                first_line = Some(line.split(",").collect::<Vec<&str>>());
                continue;
            }

            let mut columns = line.split(",");

            match first_line {
                None => unreachable!(),
                Some(ref first) => {
                    for key in first {
                        match columns.next() {
                            None => return false,
                            Some(col) => {
                                if col.len() != key.len() {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }

        true
    }

    pub fn get_header_keys(input: &str) -> Vec<&str> {
        input.lines()
            .skip_while(|row| row.starts_with("#"))
            .next().unwrap()
            .split(",").collect()
    }
}
