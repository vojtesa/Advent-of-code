use std::fs::File;
use std::io::{BufRead, BufReader};
use num_traits::PrimInt;

pub struct Grid{
    pub grid:Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl Grid{
    pub fn from_reader(data: BufReader<File>) -> Result<Self, std::io::Error>{
        let mut grid:Vec<char> = Vec::new();
        let mut lines_iter = data.lines();
        let mut height:usize = 0;
        let width:usize = if let Some(first_line_result) = lines_iter.next(){
            let first_line = first_line_result?;
            grid.extend(first_line.chars());
            height+=1;
            first_line.len()
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "First line could not be read."
            ))
        };

        for line_result in lines_iter{
            let line = line_result?;
            height += 1;
            grid.extend(line.chars())
        }

        Ok(Self{grid, width, height})
    }

    pub fn get_char_at_position<T: PrimInt>(&self, x:T, y:T) -> Option<&char>
    where
        T: TryInto<usize> + Copy
    {
        let usize_x:usize = x.try_into().ok()?;
        let usize_y:usize = y.try_into().ok()?;
        if usize_x >= self.width || usize_y >= self.height{
            return None
        }
        let absolute_position = usize_y * 139 + usize_x;
        self.grid.get(absolute_position)
    }

    pub fn set_x_at_position(&mut self, x:usize, y:usize){
        let absolute_position = y * 139 + x;
        self.grid[absolute_position] = 'x';
    }

}