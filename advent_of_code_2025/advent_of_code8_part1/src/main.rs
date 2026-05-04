#![allow(warnings)]
mod junction_box;
mod dsu;

use std::fs::File;
use std::io::{BufReader, Read};
use itertools::{all, Itertools};
use crate::dsu::DSU;
use crate::junction_box::JunctionBox;



fn load_file() -> Vec<JunctionBox> {
    let file = File::open("data.txt");
    let mut file_string = String::new();
    file.unwrap().read_to_string(&mut file_string).unwrap();

    create_junction_boxes(&mut file_string)
}

fn create_junction_boxes(file_string: &mut String) -> Vec<JunctionBox> {
    let mut junction_boxes: Vec<JunctionBox> = Vec::new();

    junction_boxes = file_string
        .lines()
        .filter_map(|line| {
            line.split(",")
                .map(|num| num.trim().parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .map(|(x, y, z)| JunctionBox::new(x, y, z))
        }).collect();


    junction_boxes
}

fn find_all_possible_pair_conn(junction_boxes: Vec<JunctionBox>) -> Vec<(usize, usize, usize)> {
    let mut all_possible_connections: Vec<(usize, usize, usize)> = Vec::new();
    let mut start_index: usize = 0;

    for index_box1 in start_index..junction_boxes.len() {
        let mut box1 = &junction_boxes[index_box1];
        for index_box2 in start_index + 1..junction_boxes.len(){
            all_possible_connections.push((index_box1, index_box2, calc_distance(*box1, junction_boxes[index_box2])));
        }
        start_index += 1;
    }

    all_possible_connections
}

fn calc_distance(box1: JunctionBox, box2: JunctionBox) -> usize {
    let mut distance: usize = box1.x.abs_diff(box2.x).pow(2) +
        box1.y.abs_diff(box2.y).pow(2) + box1.z.abs_diff(box2.z).pow(2);
    distance
}

fn sort_junctions(all_possible_pair_conn:&mut Vec<(usize, usize, usize)>) -> () {
    all_possible_pair_conn.sort_unstable_by(|a, b| a.2.cmp(&b.2));
}


fn union_connections_and_find_answer(all_possible_pair_conn:& Vec<(usize, usize, usize)>) -> usize {
    let mut my_dsu = DSU::new(2000);
    for index in 0..1000 {
        let curr_pair:(usize, usize, _) = all_possible_pair_conn[index];
        my_dsu.union(curr_pair.0, curr_pair.1);
    }
    let size_len = my_dsu.size.len();
    let (_, _, biggest_triplet) = my_dsu.size.select_nth_unstable(size_len - 4);
    biggest_triplet.iter().product()
}


fn main() {
    let junction_boxes: Vec<JunctionBox> = load_file();
    let mut all_possible_pair_conn: Vec<(usize, usize, usize)> = find_all_possible_pair_conn(junction_boxes);
    sort_junctions(&mut all_possible_pair_conn);
    let answer = union_connections_and_find_answer(&all_possible_pair_conn);
    println!("product of the 3 biggest circuits is: {answer}")

}

