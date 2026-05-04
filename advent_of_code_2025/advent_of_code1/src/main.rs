use std::{fs::{self, File}, io::{self, BufRead}};


fn load_sequence(instructions: &mut Vec<String>) -> () {
    let file = File::open("sequence.txt").expect("File does not exist");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(nth_line) => instructions.push(nth_line),
            Err(e) => eprintln!("Error while loading instruction from file: {}", e),
            }
    }
}

fn count_zeros(instructions:& Vec<String>, final_passwd: &mut i32, mut dial: i32) -> () {
    for instruction in instructions{
        if instruction.is_empty(){
            break;
        }
        let direction: char = instruction.chars().next().unwrap_or('\0');
        let number: i32 = instruction[1..].parse().unwrap_or(0);
        let mut dial_IS_zero:bool = false;
        if dial == 0 {
            dial_IS_zero = true;
        }
        if (direction == 'R'){
            dial += number;
        }
        else if (direction == 'L'){
            dial -= number;
        }
        else{
            eprintln!("Error in else block");
            break;
        }

        if dial > 0{
            *final_passwd += dial / 100;
        }
        else if dial <= 0{
            *final_passwd += -(dial / 100);
            if !dial_IS_zero{
                *final_passwd += 1;
            }
        }



        dial = ((dial % 100) + 100 ) % 100;



    }
}

fn main(){
    let dial = 50;
    let mut final_passwd = 0;
    let mut instructions: Vec<String> = Vec::new();

    load_sequence(&mut instructions);

    count_zeros(&instructions, &mut final_passwd, dial.clone());

    // for i in instructions{
    //     println!("{}", i);
    // }

    println!("{}", final_passwd);


}
