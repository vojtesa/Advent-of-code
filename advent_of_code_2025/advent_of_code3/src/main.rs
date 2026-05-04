use std::char::from_digit;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn load_file(battery_bank: &mut Vec<String>) -> Result<(), Error> {
    let file_name = "data.txt";
    let file = File::open(file_name)?;
    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        battery_bank.push(line?);
    }
    return Ok(());
}

fn fill_in_the_pairs(biggest_pairs_of_batteries: &mut Vec<[u32;12]>, battery_bank: &Vec<String>) -> () {
    for line in battery_bank.iter() {
        biggest_pairs_of_batteries.push(find_the_biggest_pair(line));
    }
}

fn find_the_biggest_pair(line:&String) -> [u32;12] {
    let mut chars_from_line:Vec<char> = line.chars().collect();
    let mut biggest_tuple:[u32;12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for index_of_nth_biggest_number in 0..12{
        let length_of_line = &chars_from_line.len();
        for digit_value in (1..=9u32).rev(){
            let digit_value_string:char = from_digit(digit_value, 10).unwrap();
            for (index, nth_character) in chars_from_line.iter().enumerate() {

                if (index + 12 > (length_of_line + index_of_nth_biggest_number)){
                    break;
                }

                if &digit_value_string == nth_character {
                    biggest_tuple[index_of_nth_biggest_number] = nth_character.to_digit(10).unwrap();
                    // if index_of_nth_biggest_number == 0 {
                    chars_from_line.drain(0..=index);
                    // }
                    break;
                }
            }
            if biggest_tuple[index_of_nth_biggest_number] == digit_value{
                break;
            }
        }
    }
    return biggest_tuple;
}




fn main() {
    let mut battery_bank: Vec<String> = Vec::new();
    load_file(&mut battery_bank);
    let mut biggest_pairs_of_batteries: Vec<[u32;12]> = Vec::new();
    fill_in_the_pairs(&mut biggest_pairs_of_batteries, &battery_bank);
    let mut answer:u128= 0;
    for (counter, i) in biggest_pairs_of_batteries.iter().enumerate(){
        // let ten:  = 10;
        for j in 0..12{
            answer += i[j] as u128 * 10u128.pow(11 - j as u32);
        }

        println!("{}{}{}{}{}{}{}{}{}{}{}{}", i[0], i[1], i[2], i[3], i[4], i[5], i[6], i[7], i[8], i[9], i[10], i[11]);
    }
    println!("Answer: {answer}");

}
