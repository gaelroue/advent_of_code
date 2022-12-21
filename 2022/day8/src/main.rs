use std::fs;

fn is_visible(x : u32, y : u32, forest : &Vec<Vec<u32>>) -> bool {
    let mut visible_top = true;
    let mut visible_bottom = true;
    let mut visible_left = true;
    let mut visible_right = true;
    
    let w = forest[y as usize].len() as u32;
    let h = forest.len() as u32;
    let ref_tree = forest[x as usize][y as usize]; 

    for i in 0..x { if forest[i as usize][y as usize] >= ref_tree {visible_left = false; break; } }
    for i in (x+1)..w { if forest[i as usize][y as usize] >= ref_tree { visible_right = false; break; } }
    for i in 0..y { if forest[x as usize][i as usize] >= ref_tree { visible_top = false; break; } }
    for i in (y+1)..h { if forest[x as usize][i as usize] >= ref_tree { visible_bottom = false; break; } }

    visible_bottom | visible_top | visible_left | visible_right
}

fn scenic_view(x : u32, y : u32, forest : &Vec<Vec<u32>>) -> u32 {
    let mut visible_top = 0;
    let mut visible_bottom = 0;
    let mut visible_left = 0;
    let mut visible_right = 0;
    
    let w = forest[y as usize].len() as u32;
    let h = forest.len() as u32;
    let ref_tree = forest[x as usize][y as usize]; 

    for i in (0..x).rev() { visible_left = visible_left + 1; if forest[i as usize][y as usize] >= ref_tree { break; } }
    for i in (x+1)..w { visible_right = visible_right + 1; if forest[i as usize][y as usize] >= ref_tree { break; } }
    for i in (0..y).rev() { visible_top = visible_top + 1; if forest[x as usize][i as usize] >= ref_tree { break; } }
    for i in (y+1)..h { visible_bottom = visible_bottom + 1; if forest[x as usize][i as usize] >= ref_tree { break; } }

    visible_bottom * visible_top * visible_left * visible_right
}

fn part_one() -> std::io::Result<()> {
    let input = fs::read_to_string("data/input.txt")?;
    let tree_map : Vec<Vec<u32>> = input.lines().map(|row| row.as_bytes().iter().map(|&tree| {(tree - 48) as u32}).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    let mut visible_trees = (tree_map[0].len() + tree_map.len()) * 2 - 4;
    for i in 1..(tree_map.len() - 1) {
        for j in 1..(tree_map[i].len() - 1) {
            if is_visible(i as u32, j as u32, &tree_map) {
                visible_trees = visible_trees + 1;
            }
        }
    }
    println!("Visible trees {}", visible_trees);

    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let input = fs::read_to_string("data/input.txt")?;
    let tree_map : Vec<Vec<u32>> = input.lines().map(|row| row.as_bytes().iter().map(|&tree| {(tree - 48) as u32}).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    let mut scenic_score = 0;
    for i in 1..(tree_map.len() - 1) {
        for j in 1..(tree_map[i].len() - 1) {
            let new_scenic_score = scenic_view(i as u32, j as u32, &tree_map);
            if new_scenic_score > scenic_score {
                scenic_score  = new_scenic_score;
            }
        }
    }
    println!("Scenic score {}", scenic_score);

    Ok(())
}

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
