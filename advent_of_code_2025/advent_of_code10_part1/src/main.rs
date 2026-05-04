mod data;

use crate::data::Data;


fn main() {
    let data: Data = Data::load_file_data();
    // data.print_content();
    let answer = data.find_answer();
    println!("Answer: {}", answer);
}
