use std::fs;

#[derive(Clone)]
struct BinString {
    bits: Vec<bool>
}
impl BinString {
    fn parse(bin_str: &str) -> BinString{
        BinString { 
            bits: bin_str.chars().map(|c| {
                match c {
                    '0' => false,
                    '1' => true,
                     _  => unreachable!("Non binary digit found in bin_str!")
                }
            }).collect()
        }
    }

    fn compliment(&self) -> BinString {
        BinString { 
            bits: self.bits.iter().map(|bit| { !bit } ).collect()
        }
    }

    fn to_dec(&self) -> usize {
        self.bits.iter().rev().enumerate().fold(0, |val, (idx, &bit)| {
            val + if bit { 2usize.pow(idx as u32) } else { 0 }
        })
    }
}

fn read_input() -> Vec<BinString> {
    let file_contents = 
        fs::read_to_string("input/day03.txt")
        .expect("Failed to read input file!");

    file_contents.lines().map(|line| {
        BinString::parse(line)
    }).collect::<Vec<BinString>>()
}

fn puzzle_one(bin_nums: &Vec<BinString>) -> usize {
    let bin_length = bin_nums[0].bits.len();
    let gamma = BinString {
        bits: (0..bin_length).map(|idx| {
            let bit_count = bin_nums.iter().fold(0isize, |count, bin_num| {
                count + if bin_num.bits[idx] { 1 } else { -1 }
            });
            if bit_count == 0 { unreachable!("Found same number of ones and zeros!"); }
            return bit_count > 0;
        }).collect()
    };

    let epsilon = gamma.compliment();
    return gamma.to_dec() * epsilon.to_dec()
}

fn filter_bin_nums(bin_nums: &Vec<BinString>, idx: usize, most_common: bool) -> Vec<BinString> {
    let bit_count = bin_nums.iter().fold(0isize, |count, bin_num| {
        count + if bin_num.bits[idx] { 1 } else { -1 }
    });
    let common_bit = bit_count >= 0;

    bin_nums.iter().filter(|bin_num| {
        bin_num.bits[idx] == (common_bit == most_common)
    }).map(|bin_num| bin_num.clone()).collect()
}

fn puzzle_two(bin_nums: &Vec<BinString>) -> usize {
    let mut oxg_bins = bin_nums.clone();
    let mut co2_bins = bin_nums.clone();

    for idx in 0..bin_nums[0].bits.len() {
        oxg_bins = filter_bin_nums(&oxg_bins, idx, true);
        if oxg_bins.len() == 1 { break; }
    }
    assert!(oxg_bins.len() == 1, "Failed to eliminate all but one binary for oxygen!");
    for idx in 0..bin_nums[0].bits.len() {
        co2_bins = filter_bin_nums(&co2_bins, idx, false);
        if co2_bins.len() == 1 { break; }
    }
    assert!(co2_bins.len() == 1, "Failed to eliminate all but one binary for co2!");

    oxg_bins[0].to_dec() * co2_bins[0].to_dec()
}

fn main() {
    let bin_nums = read_input();

    println!("Puzzle 1: {}", puzzle_one(&bin_nums));
    println!("Puzzle 2: {}", puzzle_two(&bin_nums));
}