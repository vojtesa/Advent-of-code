mod coordinate;
mod file_helper;
mod manifold;

use file_helper::load_file;


fn main() {
    let manifold_obj = load_file();
    let answer: usize = manifold_obj.count_splits();
    println!("{}", answer);
}
