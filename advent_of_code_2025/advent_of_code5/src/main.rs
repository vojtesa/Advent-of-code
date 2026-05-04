mod interval;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use interval::Interval;

fn load_from_file() -> (Vec<u64>, Vec<Interval>) {
    let file = File::open("data.txt");
    let buffer = BufReader::new(file.unwrap());
    let mut lines = buffer.lines();
    let mut available_ingredients:Vec<u64> = Vec::new();
    let mut unsorted_intervals :Vec<Interval>= Vec::new();

    for line_result in &mut lines{   //Filling unsorted_intervals
        let line = line_result.unwrap();
        if line == "" { //break if line in data file is empty
            break;
        }
        let mut line_iter = line.split("-");
        unsorted_intervals.push(Interval{
            start: line_iter.next().unwrap().parse::<u64>().unwrap(),
            end: line_iter.next().unwrap().parse::<u64>().unwrap()
        })
    }

    for line_result in &mut lines{   //Filling available_ingredients
        let line = line_result.unwrap();
        available_ingredients.push(line.parse::<u64>().unwrap());
    }


    (available_ingredients, optimize_intervals(unsorted_intervals))
}

fn optimize_intervals(mut unsorted_intervals: Vec<Interval>) -> Vec<Interval> {
    unsorted_intervals.sort_unstable_by_key(|interval| interval.start);
    unsorted_intervals
}

fn unite_intervals(sorted_intervals:Vec<Interval> ) -> Vec<Interval> {
    let mut united_intervals :Vec<Interval>= Vec::with_capacity(sorted_intervals.len());
    let mut sorted_intervals_iter = sorted_intervals.into_iter();
    let mut current_interval = sorted_intervals_iter.next().unwrap();

    for next_interval in sorted_intervals_iter {
        if current_interval.end < next_interval.start {
            united_intervals.push(current_interval.clone());
            current_interval = next_interval;
        }
        else{
            if current_interval.end < next_interval.end {
                current_interval.end = next_interval.end;
            }
        }
    }

    united_intervals.push(current_interval);
    united_intervals
}


fn get_list_of_fresh_ingredients(available_ingredients:&Vec<u64>,united_intervals:&Vec<Interval>) -> Vec<u64> {
    let mut list_of_fresh_ingredients :Vec<u64> = Vec::new();
    for ingredient in available_ingredients {
        if is_avail_ingredient_fresh(&ingredient, &united_intervals){
            list_of_fresh_ingredients.push(ingredient.clone());
        }
    }
    list_of_fresh_ingredients
}

fn is_avail_ingredient_fresh(ingredient:&u64, united_intervals:&Vec<Interval>) -> bool {
    for interval in united_intervals {
        if *ingredient <= interval.end{
            if *ingredient >= interval.start {
                return true;
            }
            else{
                return false;
            }
        }
    }
    false
}

fn sum_fresh_IDs_from_intervals(united_intervals:&Vec<Interval>) -> u64 {
    let mut sum:u64 = 0;
    for interval in united_intervals {
        sum += interval.end - interval.start + 1;
    }

    sum
}

fn main() {
    let (mut available_ingredients, mut sorted_intervals) = load_from_file();
    let mut united_intervals:Vec<Interval> = unite_intervals(sorted_intervals);
    let list_of_fresh_ingredients = get_list_of_fresh_ingredients(&available_ingredients, &united_intervals);

    println!("Available ingredients: {:?}", sum_fresh_IDs_from_intervals(&united_intervals));

//     for i in united_intervals {
//         println!{"{} - {}", i.start, i.end}
//     }
//     for i in available_ingredients {
//         println!("{} is available", i);
//     }
}
