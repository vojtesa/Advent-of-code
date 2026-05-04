use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Data{
    pub buttons_nums_groups: Vec<Vec<Vec<u32>>>,    //no purpose, just keeping it
    pub all_joltages: Vec<Vec<u32>>,
}

impl Data {
    pub fn load_file_data() -> Self {
        let file = File::open("data.txt").unwrap();
        let reader = BufReader::new(file);


        let mut buttons_nums_groups: Vec<Vec<Vec<u32>>> = Vec::new();
        let mut all_joltages: Vec<Vec<u32>> = Vec::new();

        for (index,line) in reader.lines().enumerate() {
            let line = line.unwrap();
            buttons_nums_groups.push(Vec::new());
            for word in line.split_whitespace() {
                match word {
                    w if w.starts_with('(') => buttons_nums_groups[index].push(
                        word.chars().filter(|c| c.is_ascii_digit())
                            .map(|c| c.to_digit(10).unwrap())
                            .collect()
                    ),
                    w if w.starts_with('{') => all_joltages.push(
                        word.trim_matches(|c| c == '{' || c == '}')
                            .split(',')
                            .map(|char_num| char_num.parse::<u32>().unwrap()
                            ).collect()
                    ),
                    _ => {}
                }
            }
        }


        Self {
            buttons_nums_groups,
            all_joltages
        }
    }

    pub fn print_content(&self) {
        println!();
        println!("toggle_nums:");
        for i in self.buttons_nums_groups.iter() {
            println!("{:?}", i);
        }
        println!();
        println!("joltage:");
        for i in self.all_joltages.iter() {
            println!("{:?}", i);
        }
        println!();
    }


    pub fn find_answer(&self) -> u32 {
        let mut min_of_toggles: Vec<u32> = Vec::new();

        for (index, desired_joltage) in self.all_joltages.iter().enumerate() {
            let init_zero_joltage = vec![0; desired_joltage.len()];
            let curr_buttons_groups_line = &self.buttons_nums_groups[index];
            let mut queue: VecDeque< (Vec<(u32)>, u32) > = VecDeque::new(); //(Vec(joltage_couter), num_of_cycles)
            queue.push_back((init_zero_joltage.clone(), 0));
            let mut memory_already_created: HashSet<Vec<u32>> = HashSet::new();
            memory_already_created.insert(init_zero_joltage);
            println!("{:?}", desired_joltage);
            while let Some(mut curr_joltage_couter) = queue.pop_front() {
                if *desired_joltage == curr_joltage_couter.0 {
                    println!("Found: {:?}", curr_joltage_couter);
                    min_of_toggles.push(curr_joltage_couter.1);
                    break;
                }
                for button_group in curr_buttons_groups_line.iter() {
                    if let Some(new_joltage_counter) = Self::increase_counter_with_buttons(&mut curr_joltage_couter,
                                                                                           button_group,
                                                                                           desired_joltage,
                                                                                           &mut memory_already_created)
                    {
                        queue.push_back(new_joltage_counter);
                    }
                }


            }


        }

        min_of_toggles.iter().sum()

    }

    fn increase_counter_with_buttons(curr_joltage_couter: &mut(Vec<u32>, u32),
                                     button_group: &Vec<u32>,
                                     desired_joltage: &Vec<u32>,
                                     memory_already_created: &mut HashSet<Vec<u32>>) -> Option<(Vec<u32>, u32)>{
        let mut new_joltage_counter: (Vec<u32>, u32) = curr_joltage_couter.clone();
        for button in button_group.iter() {
            new_joltage_counter.0[*button as usize] += 1;
        }
        if memory_already_created.contains(&new_joltage_counter.0) {
            return None
        }
        if !Self::check_if_counter_exceeded(desired_joltage, &new_joltage_counter.0){
            new_joltage_counter.1 += 1;
            memory_already_created.insert(new_joltage_counter.0.clone());
            return Some(new_joltage_counter)
        }
        None
    }

    fn check_if_counter_exceeded(desired_joltage: &Vec<u32>, new_joltage_counter: &Vec<u32> ) -> bool {
        for index in 0..new_joltage_counter.len(){
            if new_joltage_counter[index] > desired_joltage[index]{
                return true
            }
        }
        false
    }


}