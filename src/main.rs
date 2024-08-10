use std::{collections::btree_map::Range, os::unix::fs::DirBuilderExt, vec};
use std::ops::Range as RealRange;

fn main() {

    let new_board = Board::default();
    new_board.display_array()

}

struct Board {
    array: Vec<Vec<i16>>
}

impl Default for Board {
    fn default() -> Self {
        Board {
            array: vec![vec![0,0,0,0],
            vec![0,0,0,0],
            vec![0,0,0,0],
            vec![0,0,0,0],                 
            ]
        }
    }
    
}
enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Board {
    fn display_array(self) {
        for array in self.array.iter() {
            let mut row_str:String = String::from("[ ");
            for element in array.iter() {
                row_str.push_str(&format!("{} ", element));
            }
            row_str.push_str("]");
            println!("{}", row_str);
        }        
    }

    fn movement(&mut self, direction: Direction) {

    }

    fn check_available_movement(&self, direction:Direction) {

    }

    fn rearrange_board(&self, direction: Direction) -> Vec<Vec<i16>> {
        let mut copy_array = self.array.clone();

        match direction {
            Direction::Up => Board::vertical_rearrange((0..4),
                                    "rev",
                                                    copy_array),
            Direction::Down => Board::vertical_rearrange((0..4),
                                "none",
                                    copy_array),
            Direction::Left => {for array in &mut copy_array {
                                    array.reverse()
                                }
                                copy_array
                                },
            Direction::Right => copy_array
        }

        
    }

    pub fn vertical_rearrange(range_rows: RealRange<usize>, 
                          range_instructions: &str,
                          array_2d: Vec<Vec<i16>>) -> Vec<Vec<i16>> {


                let vec_range: Vec<usize>;
                if range_instructions == "rev" {
                    vec_range = range_rows.rev().collect();
                } else {
                    vec_range = range_rows.collect();
                }
     
                let mut new_array: Vec<Vec<i16>> = Vec::new();
                let mut array: Vec<i16> = Vec::new(); 
                // iterate over columns of the copy_array              
                for c in 0..4 {
                    // iterate over rows of the copy_array
                    for r in &vec_range {
                        array.push(array_2d[c][*r].clone())
                    }
                    new_array.push(array.clone());
                    array.clear();    
                }
                new_array
        
    }
}

//     let mut board = String::from( "+------+------+------+------+
// |      |      |      |      |
// +------+------+------+------+
// |      |      |      |      |
// +------+------+------+------+
// |      |      |      |      |
// +------+------+------+------+
// |      |      |      |      |
// +------+------+------+------+
// ")

    // println!("{}", board);  

    // let str_new_board = format!("{}{}{}", 
    //                 &board[0..100],
    //                 "2", 
    //                 &board[101..]);

    // let board = String::from(str_new_board);

    // println!("{}", board);