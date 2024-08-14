use core::panic;
use rand::Rng;
use std::ops::Range;
use std::vec;

fn main() {
    let mut new_board = Board::default();
    new_board.display_array();
    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let mut rng = rand::thread_rng();
    for _i in 0..60 {
        let random_index = rng.gen_range(0..directions.len());
        match &directions[random_index] {
            Direction::Up => println!("up"),
            Direction::Down => println!("down"),
            Direction::Left => println!("left"),
            Direction::Right => println!("right"),
            _ => panic!("Unexpected Error"),
        }

        new_board.movement(directions[random_index]);
        new_board.display_array();
    }
}

struct Board {
    array: Vec<Vec<i16>>,
}

impl Default for Board {
    fn default() -> Self {
        let mut initialize_array:Vec<Vec<i16>> = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        Board::static_array_default_generator(&mut initialize_array);

        Board {
            array: initialize_array,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    UndoUp,
}

impl Board {
    pub fn display_array(&self) {
        for array in self.array.iter() {
            let mut row_str: String = String::from("[ ");
            for element in array.iter() {
                row_str.push_str(&format!("{} ", element));
            }
            row_str.push_str("]");
            println!("{}", row_str);
        }
        println!()
    }

    pub fn movement(&mut self, direction: Direction) {
        let mut rearranged_board =
            Board::static_rearrange_board(&mut self.array.clone(), &direction);
        let rearranged_board_copy = rearranged_board.clone();
        for array in &mut rearranged_board {
            let mut num_of_zeroes = array.iter().filter(|&&x| x == 0).count();
            array.retain(|&x| x != 0);
            for i in (1..array.len()).rev() {
                if array[i] == array[i - 1] {
                    array[i] = array[i] * 2;
                    array[i - 1] = 0;
                    array.retain(|&x| x != 0);
                    num_of_zeroes += 1;
                }
            }


            for _ in 0..num_of_zeroes {
                array.insert(0, 0);
            }
        }

        // if this expresion is false, there were no changes because
        // an impossible movement was attempted
        // so there shouldn't be a random 2
        if rearranged_board_copy != rearranged_board {
            // locate the zeroes for add a random 2
            let mut zeroes_positions: Vec<Vec<usize>> = vec![];
            for r in 0..4 {
                for c in 0..4 {
                    if rearranged_board[r][c] == 0 {
                        *&mut zeroes_positions.push(vec![r, c]);
                    }
                }
            }

            // add a random 2
            if zeroes_positions.len() > 0 {
                let mut rng = rand::thread_rng();
                let random_index = rng.gen_range(0..zeroes_positions.len());
                let selected_position = &zeroes_positions[random_index];
                *&mut rearranged_board[selected_position[0]][selected_position[1]] = 2;
            }
        }

        if direction != Direction::Up {
            *&mut self.array = Board::static_rearrange_board(&mut rearranged_board, &direction);
        } else {
            let new_direction = Direction::UndoUp;
            *&mut self.array = Board::static_rearrange_board(&mut rearranged_board, &new_direction);
        }
    }

    fn static_rearrange_board(array: &mut Vec<Vec<i16>>, direction: &Direction) -> Vec<Vec<i16>> {
        let mut copy_array = array.clone();

        match *direction {
            Direction::Up => Board::static_vertical_rearrange(0..4, "rev", copy_array, false),
            Direction::Down => Board::static_vertical_rearrange(0..4, "none", copy_array, false),
            Direction::Left => {
                for array in &mut copy_array {
                    array.reverse()
                }
                copy_array
            }
            Direction::Right => copy_array,
            Direction::UndoUp => Board::static_vertical_rearrange(0..4, "rev", copy_array, true),
        }
    }

    fn static_vertical_rearrange(
        range_rows: Range<usize>,
        range_instructions: &str,
        array_2d: Vec<Vec<i16>>,
        undo_up_check: bool,
    ) -> Vec<Vec<i16>> {
        let mut array_2d = array_2d.clone();
        let vec_range: Vec<usize>;
        if range_instructions == "rev" {
            vec_range = range_rows.rev().collect();
        } else {
            vec_range = range_rows.collect();
        }

        let mut new_array: Vec<Vec<i16>> = Vec::new();
        let mut array: Vec<i16> = Vec::new();

        let undo_up_checker = if undo_up_check { 3 } else { 1 };

        for _ in 0..undo_up_checker {
            new_array.clear();
            // iterate over columns of the copy_array
            for c in 0..4 {
                // iterate over rows of the copy_array
                for r in &vec_range {
                    array.push(array_2d[*r][c].clone())
                }
                new_array.push(array.clone());
                array.clear();
            }
            array_2d = new_array.clone();
        }

        new_array
    
    
    }

    fn static_array_default_generator(array:&mut Vec<Vec<i16>>) {
        let mut rng = rand::thread_rng();
        let first_array = vec![rng.gen_range(0..4), rng.gen_range(0..4)];
        let mut second_array = vec![rng.gen_range(0..4), rng.gen_range(0..4)];
        while first_array == second_array {
            second_array = vec![rng.gen_range(0..4), rng.gen_range(0..4)];
        }

        array[first_array[0]][first_array[1]] = 2;
        array[second_array[0]][second_array[1]] = 2;
        

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
