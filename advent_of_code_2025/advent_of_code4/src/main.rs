mod grid;



use std::fs::File;
use std::io::BufReader;
use crate::grid::Grid;

fn load_file() -> BufReader<File>{
    let file = File::open("data.txt");
    let buffer = BufReader::new(file.expect("Could not open file"));
    buffer
}

fn run_loop_through_grid(grid1: &mut Grid, no_change:&mut bool) -> usize{
    let mut accessible_rolls = 0;
    for y in 0..grid1.height {
        for x in 0..grid1.width{
            let Some(current_position) = grid1.get_char_at_position(x, y).filter(|&&c| c == '@') else {continue};
            if are_there_max_3_rolls(&grid1, x, y){
                *no_change = false;
                accessible_rolls += 1;
                grid1.set_x_at_position(x, y);
            }
        }
    }
    accessible_rolls
}

fn are_there_max_3_rolls(grid1:&Grid, x:usize, y:usize) -> bool{
    let offset:[isize; 3] = [-1, 0, 1];
    let mut number_of_ampersands = 0;
    for y_offset in offset.iter(){
        for x_offset in offset.iter(){
            if grid1.get_char_at_position(x as isize + x_offset, y as isize + y_offset) == Some(&'@'){
                if *x_offset == 0 && *y_offset == 0{
                    continue;
                }
                number_of_ampersands += 1;
            }
        }
    }
    if number_of_ampersands < 4 {
        return true;
    }
    false
}


fn remove_all_removable_rolls(grid1: &mut Grid) -> usize{
    let mut sum = 0;
    let mut no_change = true;
    loop{
        no_change = true;
        sum += run_loop_through_grid(grid1, &mut no_change);

        if no_change{
            return sum;
        }
    }
}


fn main() {
    let mut grid1 = grid::Grid::from_reader(load_file()).expect("Could not load grid");
    let answer = remove_all_removable_rolls(&mut grid1);;

    println!("Answer: {}", answer);
    for (index, i) in grid1.grid.iter().enumerate() {
        if index % grid1.width == 0 {
            println!("");
        }
        print!("{}", i);
    }
}









