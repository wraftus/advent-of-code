use std::fs;
use std::iter::zip;

#[derive(Debug)]
enum PacketItem {
    Number(usize),
    List(Vec<PacketItem>)
} impl PacketItem {
    // compare two packet items together
    fn compare_order(left_item: &PacketItem, right_item: &PacketItem) -> Option<bool> {
        match left_item {
            PacketItem::Number(left_n) => match right_item {
                PacketItem::Number(right_n) => {
                    // number to number, check if the left is small
                    if left_n < right_n { return Some(true); }
                    if left_n > right_n { return Some(false); }
                }
                PacketItem::List(_) => {
                    // number to list, convert number to list and try again
                    let left_item = Self::List(vec![Self::Number(*left_n)]);
                    return Self::compare_order(&left_item, right_item);
                }
            }
            PacketItem::List(left_l) => match right_item {
                PacketItem::Number(right_n) => {
                    // number to list, convert number to list and try again
                    let right_item = Self::List(vec![Self::Number(*right_n)]);
                    return Self::compare_order(left_item, &right_item);
                }
                PacketItem::List(right_l) => {
                    // list to list, compare items one by one
                    for (left_item, right_item) in zip(left_l, right_l) {
                        match Self::compare_order(left_item, right_item) {
                            Some(b) => { return Some(b); },
                            None    => {}
                        }
                    }
                    // list comparison was inconclusive, check list lengths
                    if left_l.len() < right_l.len() { return Some(true); }
                    if left_l.len() > right_l.len() { return Some(false); }
                }
            }
        }
        // comparison was inconclusive
        None
    }
}

// break a list (vec of chars) by commas, ignoring recursive lists
fn split_list(chars: &Vec<char>) -> Vec<Vec<char>> {
    if !(*chars.first().unwrap() == '[') || !(*chars.last().unwrap() == ']') {
        panic!("Given char vec is not of form [...]!");
    }

    let mut bracket_count: usize = 0;
    let mut split_chars: Vec<Vec<char>> = Vec::new();
    for chr in &chars[1..(chars.len()-1)] {
        match chr {
            '[' => {
                bracket_count += 1;
                match split_chars.last_mut() {
                    None    => { split_chars.push(vec![*chr]); }
                    Some(v) => { v.push(*chr); }
                }
            },
            ']' => {
                if bracket_count == 0 { panic!("Unmatched closing brace!"); }
                bracket_count -= 1;
                split_chars.last_mut().unwrap().push(*chr);
            },
            ',' => {
                if bracket_count == 0 {
                    split_chars.push(Vec::new());
                } else {
                    split_chars.last_mut().unwrap().push(*chr);
                }
            },
            _ => {
                match split_chars.last_mut() {
                    None    => { split_chars.push(vec![*chr]); }
                    Some(v) => { v.push(*chr); }
                }
            }
        }
    }
    split_chars
}

fn parse_item(item_chars: &Vec<char>) -> PacketItem {
    let item_string = item_chars.iter().collect::<String>();
    match item_string.parse::<usize>() {
        Ok(num) => PacketItem::Number(num),
        Err(..) => {
            let split_chars = split_list(item_chars);
            let items = split_chars.iter().map(parse_item).collect::<Vec<PacketItem>>();
            PacketItem::List(items)
        }
    }
}

fn read_input() -> Vec<PacketItem> {
    let file_contents =
        fs::read_to_string("input/day13.txt")
        .expect("Failed to read input file!");

    file_contents.lines().collect::<Vec<&str>>()
        .chunks(3).fold(Vec::new(), |mut packets, lines| {
            assert!(lines.len() < 3 || lines[2].is_empty(), "Poorly formed packet chunk!");

            // we treat the full packet as a list packet item
            packets.push(parse_item(&lines[0].chars().collect::<Vec<char>>()));
            packets.push(parse_item(&lines[1].chars().collect::<Vec<char>>()));
            packets
        })
}

fn puzzle_one(packets: &Vec<PacketItem>) -> usize {
    packets.chunks(2).enumerate()
    .fold(0, |sum, (pair_idx, packets)| {
        let left_packet = packets.get(0).unwrap();
        let right_packet = packets.get(1).unwrap();
        match PacketItem::compare_order(left_packet, right_packet) {
            Some(correct_order) => {
               return sum + (if correct_order { pair_idx + 1 } else { 0 });
            }
            None => {
                unreachable!("Comparison of packets was inconclusive!");
            }
        }
    })
}

fn insert_jdx(curr_packet: &PacketItem, packets: &Vec<PacketItem>, sorted_idxs: &[usize]) -> usize {
    let mut insert_jdx: Option<usize> = None;
    for (jdx, other_idx) in sorted_idxs.iter().enumerate() {
        let othr_packet = packets.get(*other_idx).unwrap();
        match PacketItem::compare_order(curr_packet, othr_packet) {
            Some(res) => {
                if res {
                    insert_jdx = Some(jdx);
                    break;
                }
            },
            None => { unreachable!("Comparison of packets was inconclusive!"); }
        }
    }
    match insert_jdx {
        Some(jdx) => jdx,
        None      => sorted_idxs.len()
    }
}

fn puzzle_two(packets: &Vec<PacketItem>) -> usize {
    // sort packet idxs in another vec (poopy insert sort, but its fast enough)
    let mut sorted_idxs: Vec<usize> = Vec::new();
    for curr_idx in 0..packets.len() {
        let curr_packet = packets.get(curr_idx).unwrap();

        let insert_jdx = insert_jdx(curr_packet, packets, &sorted_idxs);
        sorted_idxs.insert(insert_jdx, curr_idx);
    }

    // see where the two divider packets
    let fst_div_pckt = PacketItem::List(vec![PacketItem::List(vec![PacketItem::Number(2)])]);
    let fst_div_jdx = insert_jdx(&fst_div_pckt, packets, &sorted_idxs);

    let snd_div_pckt = PacketItem::List(vec![PacketItem::List(vec![PacketItem::Number(6)])]);
    let idx_check_slice = &sorted_idxs[fst_div_jdx..];
    let snd_div_jdx = insert_jdx(&snd_div_pckt, packets, idx_check_slice) + (fst_div_jdx + 1);

    (fst_div_jdx + 1) * (snd_div_jdx + 1)
}

fn main() {
    let packets: Vec<PacketItem> = read_input();

    println!("Puzzle 1: {}", puzzle_one(&packets));
    println!("Puzzle 2: {}", puzzle_two(&packets));
}