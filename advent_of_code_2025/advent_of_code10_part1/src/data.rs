use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Data{
    pub lights_pattern_binary: Vec<(u32, u32)>,  //(binary_num, width_of_light_pattern)
    pub toggle_nums: Vec<Vec<Vec<u32>>>,    //no purpose, just keeping it
    pub toggle_nums_binary: Vec<Vec<u32>>,
    pub joltage: Vec<Vec<u32>>,
}

impl Data {
    pub fn load_file_data() -> Self {
        let file = File::open("data.txt").unwrap();
        let reader = BufReader::new(file);

        let mut lights_pattern_binary: Vec<(u32, u32)> = Vec::new(); //(binary_num, width_of_light_pattern)
        let mut toggle_nums: Vec<Vec<Vec<u32>>> = Vec::new();
        let mut toggle_nums_binary: Vec<Vec<u32>> = Vec::new();
        let mut joltage: Vec<Vec<u32>> = Vec::new();

        for (index,line) in reader.lines().enumerate() {
            let line = line.unwrap();
            toggle_nums.push(Vec::new());
            for word in line.split_whitespace() {
                match word {
                    w if w.starts_with('[') => lights_pattern_binary.push(
                        word.trim_matches(|c| c == '[' || c == ']')
                            .chars().fold((0u32, 0u32), |(acc, count), c| {
                            let bit = if c == '#' { 1 } else { 0 };
                            ((acc << 1) | bit, count + 1)
                        }
                        )
                    ),
                    w if w.starts_with('(') => toggle_nums[index].push(
                        word.chars().filter(|c| c.is_ascii_digit())
                            .map(|c| c.to_digit(10).unwrap())
                            .collect()
                    ),
                    w if w.starts_with('{') => joltage.push(
                        word.trim_matches(|c| c == '{' || c == '}')
                            .split(',')
                            .map(|char_num| char_num.parse::<u32>().unwrap()
                            ).collect()
                    ),
                    _ => {}
                }
            }
        }

        for (index_of_lights_pattern,line) in toggle_nums.iter().enumerate() {
            let mut toggle_nums_on_one_line: Vec<u32> = Vec::new();
            for toggle_nums_iter in line.iter() {
                toggle_nums_on_one_line.push(Self::num_to_binary_num(toggle_nums_iter, lights_pattern_binary[index_of_lights_pattern].1));
            }
            toggle_nums_binary.push(toggle_nums_on_one_line);
        }


        Self {
            lights_pattern_binary,
            toggle_nums,
            toggle_nums_binary,
            joltage
        }
    }

    pub fn print_content(&self) {
        println!("lights_pattern_binary:");
        for i in self.lights_pattern_binary.iter() {
            println!("{:b},{}", i.0, i.1);
        }
        println!();
        println!("toggle_nums:");
        for i in self.toggle_nums.iter() {
            println!("{:?}", i);
        }
        println!();
        println!("toggle_nums_binary binary output:");
        for i in self.toggle_nums_binary.iter() {
            for j in i.iter() {
                print!("{:b} ", j);
            }
            println!();
        }
        println!();
        println!("toggle_nums_binary DEBUG output:");
        for i in self.toggle_nums_binary.iter() {
            println!("{:?}", i);
        }
        println!();
        println!("joltage:");
        for i in self.joltage.iter() {
            println!("{:?}", i);
        }
        println!();
    }

    fn num_to_binary_num(array_of_nums: &Vec<u32>, num_of_digits: u32) -> u32{
        let mut binary_num: u32 = 0;
        for num in array_of_nums {
            binary_num = (1 << (num_of_digits - 1 - num)) ^ binary_num;
        }
        binary_num
    }

    pub fn find_answer(&self) -> u32 {
        let mut min_of_toggles: Vec<u32> = Vec::new();

        for index in 0..self.lights_pattern_binary.len() {
            let mut tried_memory: HashSet<u32> = HashSet::new();
            let curr_wanted_pattern = &self.lights_pattern_binary[index];
            let curr_toggle_nums = &self.toggle_nums_binary[index];
            let mut queue: VecDeque<(u32,u32)> = VecDeque::new(); //VecDeque<(lights_pattern, num_of_toggled_buttons)>
            queue.push_back((0, 0)); tried_memory.insert(0);


            while !queue.is_empty() {
                let curr_light_pattern = queue.pop_front().unwrap();

                if curr_light_pattern.0 ^ curr_wanted_pattern.0 == 0 {
                    min_of_toggles.push(curr_light_pattern.1);
                    break;
                }
                for curr_toggle_num in curr_toggle_nums {
                    let new_light_pattern = (curr_toggle_num ^ curr_light_pattern.0);
                    if !tried_memory.contains(&new_light_pattern){
                    queue.push_back((new_light_pattern, curr_light_pattern.1 + 1));
                    tried_memory.insert(new_light_pattern);
                    }
                }
            }
        }


        min_of_toggles.iter().sum()
    }


}