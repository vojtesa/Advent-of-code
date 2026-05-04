use std::fs;
use std::fs::File;

fn load_form_file(ranges: &mut Vec<[u64; 2]>) -> () {
    let mut file = fs::read_to_string("data.txt").expect("Unable to read file");
    ranges.extend(file
                      .trim()
                      .split(',')
                      .map(|pair| {
                          let mut nums = pair
                              .split('-')
                              .map(|s| s.parse::<u64>().unwrap());
                          [nums.next().unwrap(), nums.next().unwrap()]
                      })
    );

}

fn find_invalid(ranges: &Vec<[u64; 2]>, invalid_ids: &mut Vec<u64>) -> () {
    let mut inter_start: u64;
    let mut inter_end: u64;
    for interval in ranges {
        inter_start = interval[0];
        inter_end = interval[1];
        for num in inter_start .. inter_end + 1 {
            if find_pattern(num) != 0 {
                invalid_ids.push(num);
            };
        }
    }
}

fn find_pattern(number: u64) -> u64{
    let string_number = number.to_string();
    let length = string_number.len();

    for end_of_slice in 1..(length / 2) + 1 {
        let chars_to_compare= &string_number[0..end_of_slice];
        let increment_step = end_of_slice;
        let mut do_slices_match = false;

        //Creates starting number for slice
        for start_of_mov_slice in (increment_step..length).step_by(increment_step){
            let mut end_of_mov_slice: usize;
            if (increment_step + start_of_mov_slice) > length{
                end_of_mov_slice = length;
            }
            else{
                end_of_mov_slice = start_of_mov_slice+increment_step;
            }

            let moving_slice = &string_number[start_of_mov_slice..end_of_mov_slice];
            if chars_to_compare !=  moving_slice {
                do_slices_match = false;
                break;
            }
            else if chars_to_compare == moving_slice {
                do_slices_match = true;
            }
            else{
                eprintln!("Unable to compare chars {:?}", chars_to_compare);
            }
        }
        if do_slices_match{
            return chars_to_compare.parse::<u64>().unwrap();
        }

    }
    return 0;
}

fn main() {
    let mut ranges:Vec<[u64;2]> = Vec::new();
    let mut invalid_ids:Vec<u64> = Vec::new();
    load_form_file(&mut ranges);
    find_invalid(&ranges, &mut invalid_ids);
    let mut result: u128= 0;
    for i in &invalid_ids {
        result += *i as u128;
    }
    println!("Sum is: {}", result);

    for i in &invalid_ids {
        println!("{}", i);
    }
}
