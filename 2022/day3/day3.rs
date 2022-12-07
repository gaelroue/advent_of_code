use std::fs;

fn part_one() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let priorities: u32 = input.lines().map(|rucksack| rucksack.split_at(rucksack.len()/2)).map(|(c1, c2)| {
            let mut common_item = 0;
            for c in c1.chars() {
                if c2.contains(c) {common_item = c as u8; break}
            }

            if common_item >= b'a' { (common_item - b'a' + 1) as u32}
            else {(common_item - b'A' + 27) as u32}
        }).sum();

    println!("The sum of priorities for those items is {}", priorities);
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let priorities: Vec<&str> = input.lines().collect();

    let badges_sum : u32 = priorities.chunks(3).map(|lines| {
            let mut common_badge = 0;
            for c in lines[0].chars() {
                if lines[1].contains(c) && lines[2].contains(c) {common_badge = c as u8; break}
            }

            if common_badge >= b'a' { (common_badge - b'a' + 1) as u32}
            else {(common_badge - b'A' + 27) as u32}
        }).sum();
             
    println!("The sum of priorities for badges is {}", badges_sum);
    Ok(())
}

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
