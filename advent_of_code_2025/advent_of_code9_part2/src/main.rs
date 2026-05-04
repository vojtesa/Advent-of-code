use std::collections::{HashSet, VecDeque};
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



fn compress_coordinates(red_tiles:& Vec<(usize, usize)>) -> (Vec<Vec<u32>>, Vec<usize>, Vec<usize>)  {
    let mut xs: Vec<usize> = red_tiles.iter().map(|(x, _)| *x).collect();
    let mut ys: Vec<usize> = red_tiles.iter().map(|(_, y)| *y).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    let height = ys.len();
    let width = xs.len();

    let mut grid = vec![vec![0u32; width]; height];

    fill_in_edges_ones_and_outside_twos(&mut grid, height, width, red_tiles, &xs, &ys);

    (grid, xs, ys)
}

fn fill_in_edges_ones_and_outside_twos(grid: &mut Vec<Vec<u32>>, height: usize, width: usize, red_tiles: &Vec<(usize, usize)>, xs: &Vec<usize>, ys: &Vec<usize>) {
    let n = red_tiles.len();
    for index in 0..n {
        let (curr_x, curr_y) = red_tiles[index];
        let (next_x, next_y) = red_tiles[(index + 1) % n];

        let cmprsd_curr_x = xs.binary_search(&curr_x).unwrap();
        let cmprsd_curr_y = ys.binary_search(&curr_y).unwrap();
        let cmprsd_next_x = xs.binary_search(&next_x).unwrap();
        let cmprsd_next_y = ys.binary_search(&next_y).unwrap();

        if cmprsd_curr_x == cmprsd_next_x {
            let min_y = cmprsd_curr_y.min(cmprsd_next_y);
            let max_y = cmprsd_curr_y.max(cmprsd_next_y);
            for y in min_y..=max_y {
                grid[y][cmprsd_curr_x] = 1;
            }
        }
        else if cmprsd_curr_y == cmprsd_next_y {
            let min_x = cmprsd_curr_x.min(cmprsd_next_x);
            let max_x = cmprsd_curr_x.max(cmprsd_next_x);
            for x in min_x..=max_x {
                grid[cmprsd_curr_y][x] = 1;
            }
        }
    }
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    for x in 0..width {
        if grid[0][x] == 0 {
            queue.push_back((x, 0));
        }
    }
    for y in 0..height {
        if grid[y][width - 1] == 0 {
            queue.push_back((width - 1, y));
        }
    }
    for x in (0..width).rev() {
        if grid[height - 1][x] == 0 {
            queue.push_back((x, height - 1));
        }
    }
    for y in (0..height).rev() {
        if grid[y][0] == 0 {
            queue.push_back((0, y));
        }
    }
    let direction:[(i32, i32);4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((curr_x, curr_y)) = queue.pop_front()  {
        if grid[curr_y][curr_x] != 0{
            continue;
        }
        grid[curr_y][curr_x] = 2; //sets 2 on the coordinates where the curr is
        for direction in &direction {
            let x_insert = curr_x as i32 + direction.0;
            let y_insert = curr_y as i32 + direction.1;
            if x_insert >= 0 && y_insert >= 0 && x_insert < width as i32 && y_insert < height as i32 && grid[y_insert as usize][x_insert as usize] == 0 {
                queue.push_back((x_insert as usize, y_insert as usize));
            }
        }
    }
}



fn create_2d_prefix_sum(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut prefix_sum_2d: Vec<Vec<u32>> = vec![vec![0u32;grid[0].len() + 1];grid.len() + 1];
    let mut curr_grid_y = 0;
    let mut curr_grid_x = 0;
    for y in 1..prefix_sum_2d.len() {
        curr_grid_y = y - 1;
        for x in 1..prefix_sum_2d[0].len() {
            curr_grid_x = x - 1;
            let is_2 = if grid[curr_grid_y][curr_grid_x] == 2 {2} else {0};
            prefix_sum_2d[y][x] += is_2 + prefix_sum_2d[y - 1][x] + prefix_sum_2d[y][x - 1] - prefix_sum_2d[y - 1][x - 1];
        }
    }
    prefix_sum_2d
}


fn find_largest_area(red_tiles: &Vec<(usize, usize)>,
                     prefix_sum_2d: &Vec<Vec<u32>>,
                     xs: &Vec<usize>,
                     ys: &Vec<usize>) -> usize {
    let mut largest_areas_with_green_tiles: Vec<usize> = Vec::new();

    let mut x_first_tile_cmprsd_index:usize;
    let mut y_first_tile_cmprsd_index:usize;
    let mut x_second_tile_cmprsd_index:usize;
    let mut y_second_tile_cmprsd_index:usize;

    for (index1, red_tile1) in red_tiles.iter().enumerate() {
        x_first_tile_cmprsd_index = xs.binary_search(&red_tile1.0).unwrap();
        y_first_tile_cmprsd_index = ys.binary_search(&red_tile1.1).unwrap();

        for index2 in index1+1..red_tiles.len() {
            x_second_tile_cmprsd_index = xs.binary_search(&red_tiles[index2].0).unwrap();
            y_second_tile_cmprsd_index = ys.binary_search(&red_tiles[index2].1).unwrap();
            let corner1 = (x_first_tile_cmprsd_index, y_first_tile_cmprsd_index);
            let corner2 = (x_second_tile_cmprsd_index, y_second_tile_cmprsd_index);
            if is_area_eq_zero(corner1, corner2, &prefix_sum_2d) {
            largest_areas_with_green_tiles.push(calc_area(*red_tile1, red_tiles[index2]));
            }
        }
    }


    largest_areas_with_green_tiles.iter().max().unwrap().clone()
}

fn is_area_eq_zero(corner1:(usize, usize), corner2:(usize, usize), prefix_sum_2d: &Vec<Vec<u32>>) -> bool {
    let x_right = corner1.0.max(corner2.0);
    let x_left = corner1.0.min(corner2.0);
    let y_up = corner1.1.min(corner2.1);
    let y_down = corner1.1.max(corner2.1);

    let top_left = (x_left + 1, y_up + 1);
    let top_right = (x_right + 1, y_up + 1);
    let bottom_right = (x_right + 1, y_down + 1);
    let bottom_left = (x_left + 1, y_down + 1);

    let area = prefix_sum_2d[bottom_right.1][bottom_right.0]
        + prefix_sum_2d[top_left.1 - 1][top_left.0 - 1]
        - prefix_sum_2d[top_right.1 - 1][top_right.0]
        - prefix_sum_2d[bottom_left.1][bottom_left.0 - 1];

    if area == 0 {
        true
    } else{
        false
    }
}

fn calc_area(red_tile1: (usize, usize), red_tile2: (usize, usize)) -> usize {
    let x = red_tile1.0.abs_diff(red_tile2.0).max(1) + 1;
    let y = red_tile2.1.abs_diff(red_tile1.1).max(1) + 1;
    x * y
}

fn main() {
    let red_tiles = load_red_tiles();
    let (red_tiles_compressed, xs, ys) = compress_coordinates(&red_tiles);
    let prefix_sum_2d = create_2d_prefix_sum(&red_tiles_compressed);
    let answer = find_largest_area(&red_tiles, &prefix_sum_2d, &xs, &ys);

    for (index, i) in red_tiles.iter().enumerate() {
        println!("{:?}", i);
        if index > 4 { break}
    }
    println!();
    for (index, i) in red_tiles_compressed.iter().enumerate() {
        println!("{:?}", i);
        if index > 4 { break}
    }

    println!();
    for (index, i) in prefix_sum_2d.iter().enumerate() {
        println!("{:?}", i);
        if index > 4 { break}
    }

    println!("Largest area is: {}", answer);
}
