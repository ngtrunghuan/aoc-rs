use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let mut max_cal: i32 = 0;
    let mut cur_cal: i32 = 0;
    let mut top_3: Vec<i32> = vec![];

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(raw) = line {
                if raw.is_empty() {
                    max_cal = std::cmp::max(max_cal, cur_cal);
                    top_3 = arrange_top_3(top_3, cur_cal);
                    cur_cal = 0;
                    continue;
                }

                if let Ok(cal) = raw.parse::<i32>() {
                    cur_cal += cal;
                }
            }
        }
    }
    println!("{}", max_cal);
    println!("{}", top_3.iter().sum::<i32>());
    println!("{:?}", top_3);
}

fn arrange_top_3(mut top_3: Vec<i32>, num: i32) -> Vec<i32> {
    println!("{:?} {}", top_3, num);
    if top_3.len() < 3 {
        top_3.extend([num].iter());
        return top_3.to_vec();
    } else {
        match top_3.iter().min() {
            Some(min_val) => {
                if num < *min_val {
                    return top_3.to_vec();
                } else {
                    top_3.extend([num].iter());
                    top_3.sort();
                    println!("{:?}", top_3);
                    return top_3[1..].to_vec();
                }
            }
            None => {
                panic!("Should not reach this");
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
