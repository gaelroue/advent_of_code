use std::fs;

fn part_one() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let overlap_tasks = input.lines() // 2-4,6-8
        .map(|pair| pair.split_once(",").unwrap()) //2-4 and 6-8
        .filter(|(elf1, elf2)| {
            let (r11, r12) = elf1.split_once("-").unwrap();
            let (r21, r22) = elf2.split_once("-").unwrap();
            filter_one(r11.parse::<u32>().unwrap(),
                    r12.parse::<u32>().unwrap(),
                    r21.parse::<u32>().unwrap(),
                    r22.parse::<u32>().unwrap())
            }).count();

    println!("Number of fully overlap : {}", overlap_tasks);
    Ok(())
}

fn filter_one (min1:u32, max1:u32, min2:u32, max2:u32) -> bool {
    (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
}

fn filter_two (min1:u32, max1:u32, min2:u32, max2:u32) -> bool {
    let mut ret = false;

    for r in min2..max2+1 {
        if (min1..max1+1).contains(&r) {ret = true; break;}
    }
    ret
}

fn part_two() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let overlap_tasks = input.lines() // 2-4,6-8
        .map(|pair| pair.split_once(",").unwrap()) //2-4 and 6-8
        .filter(|(elf1, elf2)| {
            let (r11, r12) = elf1.split_once("-").unwrap();
            let (r21, r22) = elf2.split_once("-").unwrap();

            filter_two(r11.parse::<u32>().unwrap(),
                    r12.parse::<u32>().unwrap(),
                    r21.parse::<u32>().unwrap(),
                    r22.parse::<u32>().unwrap())
            }).count();

    println!("Number of overlaped range : {}", overlap_tasks);
    Ok(())
}

 

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
