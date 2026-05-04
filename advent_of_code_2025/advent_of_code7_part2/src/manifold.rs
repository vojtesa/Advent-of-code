use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;
use crate::coordinate::Coordinate;

pub struct Manifold {
    pub start_coordinate: Coordinate,
    pub width: usize,
    pub height: usize,
    pub splitters: HashSet<Coordinate>,
}

impl Manifold {
    pub fn new(file_string: String) -> Manifold {
        let mut splitters = HashSet::new();
        let width = file_string.lines().next().map_or(0, |line| line.len());   //with first iterator get len of a line
        let mut lines = file_string.lines().enumerate();     //with second iterator do the rest
        let (y_index, first_line) = lines.next().unwrap();

        let start_coordinate : Coordinate = first_line
            .chars().enumerate()
            .find(|(_, c)| *c == 'S')
            .map(|(index, _)| Coordinate{x: index, y: 0}).unwrap();
        let mut height = 1;

        for (y_index, line) in lines {
            splitters.extend(line
                .chars().enumerate()
                .filter(|(_, c)| *c == '^')
                .map(|(x_index, _)| Coordinate{x: x_index, y: y_index}
                )
            );
            height += 1;
        }

        Self{
            start_coordinate: start_coordinate,
            width: width,
            height: height,
            splitters: splitters
        }
    }


    pub fn count_all_paths(&self) -> u128 {

        let mut current_wave:HashMap<Coordinate, u128> = HashMap::with_capacity(self.width);
        let mut next_wave:HashMap<Coordinate, u128> = HashMap::with_capacity(self.width);
        current_wave.insert(self.start_coordinate.clone(), 1);

        for i in 0..self.height {
            next_wave.clear();

            for (coord, count) in &current_wave {
                let next_y = coord.y + 1;

                if next_y >= self.height { continue; }

                let target_pos = Coordinate{x: coord.x, y: coord.y + 1};

                if self.splitters.contains(&target_pos){
                    let split_right = &Coordinate{x: coord.x + 1, y: coord.y + 1};
                    let split_left = &Coordinate{x: coord.x - 1, y: coord.y + 1};

                    *next_wave.entry(*split_right).or_insert(0) += count;
                    *next_wave.entry(*split_left).or_insert(0) += count;
                }
                else{
                    *next_wave.entry(target_pos).or_insert(0) += count;
                }
            }
            if !next_wave.is_empty() {
                swap(&mut current_wave, &mut next_wave);
            }
        }
        current_wave.values().sum()
    }

}



