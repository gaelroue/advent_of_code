use std::fs;
use std::collections::VecDeque;

extern crate regex;
use regex::Regex;

fn part_one() -> std::io::Result<()> {
    let input = fs::read_to_string("data/input.txt")?;

// ---------------------------- PARSE DOCKS -----------------------------------

    let re_end = Regex::new(r"^\s\d.*\s(\d+)\s$").unwrap();
    let mut x = 0;
    for line in input.lines() {
        let dock_base = re_end.captures(line);
        if !dock_base.is_none() {
            x = dock_base.unwrap().get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap()
        }
    }

    let mut docks : Vec<VecDeque<char>> = Vec::with_capacity(x);

    for _ in 0..x {
        docks.push(VecDeque::<char>::new());
    }

// -------------------------------------------------------------------------

    let re_move = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for line in input.lines() {
        for i in (1..line.len()).step_by(4) {
            let dock_base = re_end.captures(line);
            // if crates stacks base OR empty line
            if !dock_base.is_none() || line.is_empty() {
                continue
            }

            let move_ret = re_move.captures(line);
            
            // if move instruction
            if !move_ret.is_none() {
                let move_val = move_ret.unwrap();
                let move_n = move_val.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
                let move_from = move_val.get(2).map_or("", |m| m.as_str()).parse::<usize>().unwrap() - 1;
                let move_to = move_val.get(3).map_or("", |m| m.as_str()).parse::<usize>().unwrap() - 1;
                for _ in 0..move_n {
                    let c = docks[move_from].pop_back().unwrap();
                    docks[move_to].push_back(c);
                }
                // /!\ Break is important, otherwise iteration contine on same line
                break;
            // if crates stacks
            } else {
                let crates = line.as_bytes()[i] as char;
                if crates != ' ' {
                    docks[(i - 1) / 4].push_front(crates);
                }
            }
        }

    }

// ----------------------------------------------------------------------------

    print!("Crate on top of each stack with CrateMover 9000: ");
    for c in docks {
        print!("{}", c.back().unwrap());
    }
    println!("");
    Ok(())
}

 

 

fn part_two() -> std::io::Result<()> {
    let input = fs::read_to_string("data/input.txt")?;

// ---------------------------- PARSE DOCKS -----------------------------------

    let re_end = Regex::new(r"^\s\d.*\s(\d+)\s$").unwrap();
    let mut x = 0;
    for line in input.lines() {
        let dock_base = re_end.captures(line);
        if !dock_base.is_none() {
            x = dock_base.unwrap().get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap()
        }
    }

    let mut docks : Vec<VecDeque<char>> = Vec::with_capacity(x);

    for _ in 0..x {
        docks.push(VecDeque::<char>::new());
    }

// -------------------------------------------------------------------------

    let re_move = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for line in input.lines() {
        for i in (1..line.len()).step_by(4) {
            let dock_base = re_end.captures(line);
            // if crates stacks base OR empty line
            if !dock_base.is_none() || line.is_empty() {
                continue
            }

            let move_ret = re_move.captures(line);
            
            // if move instruction
            if !move_ret.is_none() {
                let move_val = move_ret.unwrap();
                let move_n = move_val.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
                let move_from = move_val.get(2).map_or("", |m| m.as_str()).parse::<usize>().unwrap() - 1;
                let move_to = move_val.get(3).map_or("", |m| m.as_str()).parse::<usize>().unwrap() - 1;
                let split_index = docks[move_from].len() - move_n;
                let mut c = docks[move_from].split_off(split_index);
                docks[move_to].append(&mut c);
                // /!\ Break is important, otherwise iteration contine on same line
                break;
            // if crates stacks
            } else {
                let crates = line.as_bytes()[i] as char;
                if crates != ' ' {
                    docks[(i - 1) / 4].push_front(crates);
                }
            }
        }
    }

// ----------------------------------------------------------------------------

    print!("Crate on top of each stack with CrateMover 9001: ");
    for c in docks {
        print!("{}", c.back().unwrap());
    }
    println!("");
    Ok(())
}

 

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
