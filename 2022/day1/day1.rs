use std::fs;

fn part_one() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let max = input.split("\n\n").map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>()).max().unwrap();
    println!("The Elf with the most calories carry {} calories.", max);  
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut calories = input.split("\n\n").map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>()).collect::<Vec<u32>>();
    calories.sort();

    let ret = calories.into_iter().rev().take(3).sum::<u32>();
    println!("The top 3 Elves carrying the most calories have a total of {} calories.", ret);   
    Ok(())
}

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
