use std::fs::File;
use std::io::{BufRead, BufReader};





fn do_it_all() -> u64{
    let file = File::open("data.txt").expect("file not found");
    let buffer = BufReader::new(file);
    let line_iter: Vec<String> = buffer.lines().map(|l| l.expect("chyba")).collect();

    let row1: Vec<char> = line_iter[0].chars().collect();
    let row2: Vec<char> = line_iter[1].chars().collect();
    let row3: Vec<char> = line_iter[2].chars().collect();
    let row4: Vec<char>= line_iter[3].chars().collect();
    let mut operators: Vec<char> = line_iter[4].chars().collect();
    operators.push(' ');

    let mut sum_of_all: u64 = 0;
    let mut number_in_current_column :u64 = 0;
    let mut numbers_in_columns: [u64;4] = [0;4];
    let mut current_operator: char = ' ';
    let mut index_counter = 0;
    for column_index in (0..row1.len()).rev() {
        let characters = vec![row1[column_index], row2[column_index], row3[column_index], row4[column_index]];
        if characters.iter().all(|&c| c == ' '){
            sum_of_all += recognize_operator(current_operator, numbers_in_columns).unwrap_or(0);
            numbers_in_columns = [0;4];
            index_counter = 0;
            continue;
        }
        let mut place_value: u64 = 1;
        for digit in characters.iter().rev() {
            if let Some(digit) = digit.to_digit(10){
                number_in_current_column += place_value * digit as u64;
                place_value *= 10;
            }
        }
        numbers_in_columns[index_counter] = number_in_current_column;
        number_in_current_column = 0;
        index_counter += 1;
        if operators[column_index] != ' '{
            if operators[column_index] == '+' {
                current_operator = '+';
            }
            else if operators[column_index] == '*' {
                current_operator = '*';
            }
        }
        if column_index == 0{
            sum_of_all += recognize_operator(current_operator, numbers_in_columns).unwrap_or(0);
        }
    }
    sum_of_all
}

fn recognize_operator(operator: char, numbers_in_columns: [u64;4]) -> Option<u64> {
    let mut sum:u64 = 0;
    if operator == '+'{
        sum = numbers_in_columns.iter().sum();
    }
    else if operator == '*'{
        sum = numbers_in_columns.iter().map(|c| if *c == 0 {1} else {*c}).product();
    }
    else {
        return None
    }
    Some(sum)
}



fn main() {
    println!("Sum is: {}", do_it_all());
}
