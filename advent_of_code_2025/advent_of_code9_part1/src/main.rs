use std::fs::{read_to_string};

fn load_red_tiles() -> Vec<(usize, usize)> {
    let file_string = read_to_string("data.txt").unwrap();
    let red_tiles:Vec<(usize, usize)> = file_string.lines().map(|s| {
        let (x_str, y_str) = s.split_once(",").unwrap();
        let x = x_str.parse::<usize>().unwrap();
        let y = y_str.parse::<usize>().unwrap();
        (x,y)
    }).collect();
    red_tiles
}


fn find_largest_area(red_tiles: Vec<(usize, usize)>) -> usize {
    let mut largest_areas: Vec<usize> = Vec::new();

    for (index1, red_tile1) in red_tiles.iter().enumerate() {
        for index2 in index1+1..red_tiles.len() {
            largest_areas.push(calc_area(*red_tile1, red_tiles[index2]));
        }
    }

    largest_areas.iter().max().unwrap().clone()
}

fn calc_area(red_tile1: (usize, usize), red_tile2: (usize, usize)) -> usize {
    let x = red_tile1.0.abs_diff(red_tile2.0).max(1) + 1;
    let y = red_tile2.1.abs_diff(red_tile1.1).max(1) + 1;
    x * y
}


fn main() {
    let red_tiles = load_red_tiles();
    let answer = find_largest_area(red_tiles);

    println!("Largest area is: {}", answer);
}
