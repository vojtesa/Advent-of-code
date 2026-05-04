use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
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

    pub fn count_splits(&self) -> usize {
        let mut total_splits = 0;

        let mut active_beams:VecDeque<Coordinate> = VecDeque::new();
        active_beams.push_back(self.start_coordinate);
        let mut visited_coordinates: HashSet<Coordinate> = HashSet::new();
        visited_coordinates.insert(self.start_coordinate);

        while !active_beams.is_empty() {
            let mut curr_beam = active_beams.pop_front().unwrap();
            if self.splitters.contains(&curr_beam) {
                total_splits += 1;
                let beam_right:Coordinate = Coordinate{x: curr_beam.x + 1, y: curr_beam.y };
                let beam_left:Coordinate = Coordinate{x: curr_beam.x - 1, y: curr_beam.y};

                if !visited_coordinates.contains(&beam_right){
                    active_beams.push_back(beam_right);
                    visited_coordinates.insert(beam_right);
                }
                if !visited_coordinates.contains(&beam_left){
                    active_beams.push_back(beam_left);
                    visited_coordinates.insert(beam_left);
                }

            }
            else if curr_beam.y == self.height{
                continue;
            }
            else {
                curr_beam.y += 1;
                if !visited_coordinates.contains(&curr_beam){
                active_beams.push_back(curr_beam);
                visited_coordinates.insert(curr_beam);
                }
            }
        }
        total_splits
    }
}