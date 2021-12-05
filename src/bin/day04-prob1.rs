#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let (order, mut boards) = std::fs::read_to_string("src/bin/day04.txt")
        .map(|file| {
            let lines = file.lines().filter(|line| !line.is_empty()).collect_vec();
            let order = lines[0]
                .split(',')
                .map(|val| val.parse::<u16>().unwrap())
                .collect_vec();
            let boards = (0..(lines.len() - 1) / 5)
                .map(|offset| Board::from_str(&lines[1 + (offset * 5)..1 + ((offset + 1) * 5)]))
                .collect_vec();
            (order, boards)
        })
        .expect("Unable to open file");
    let winner = play_game(order, &mut boards);
    println!("{:?}", winner);
}

#[derive(Debug, PartialEq)]
struct Board {
    board: Vec<Vec<bool>>,
    indices: HashMap<u16, (usize, usize)>,
}

impl Board {
    fn from_str(lines: &[&str]) -> Board {
        let parsed = lines
            .iter()
            .map(|line| {
                vec![
                    line[0..2].trim().parse::<u16>().unwrap(),
                    line[3..5].trim().parse::<u16>().unwrap(),
                    line[6..8].trim().parse::<u16>().unwrap(),
                    line[9..11].trim().parse::<u16>().unwrap(),
                    line[12..14].trim().parse::<u16>().unwrap(),
                ]
            })
            .collect_vec();

        let mut indices: HashMap<u16, (usize, usize)> = HashMap::new();
        for (x, row) in parsed.iter().enumerate() {
            for (y, val) in row.iter().enumerate() {
                indices.insert(*val, (x, y));
            }
        }

        Board::new(indices)
    }

    fn new(indices: HashMap<u16, (usize, usize)>) -> Board {
        Board {
            board: vec![vec![false; 5]; 5],
            indices,
        }
    }

    fn mark(&mut self, val: u16) {
        if let Some((x, y)) = self.indices.get(&val) {
            self.board[*x][*y] = true;
        }
    }

    fn is_bingo(&self) -> bool {
        for i in 0..5 {
            if self.board[i].iter().all(|x| *x) || self.board.iter().all(|row| row[i]) {
                return true;
            }
        }
        false
    }

    fn calc_score(&self) -> u32 {
        self.indices
            .iter()
            .filter_map(|(val, (x, y))| {
                if self.board[*x][*y] {
                    None
                } else {
                    Some(*val as u32)
                }
            })
            .sum()
    }
}

fn play_game(order: Vec<u16>, boards: &mut Vec<Board>) -> u32 {
    for val in order {
        for board in &mut *boards {
            board.mark(val);
            if board.is_bingo() {
                return board.calc_score() * val as u32;
            }
        }
    }
    panic!("Unable to find winning board after playing all values");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board_from_str() {
        assert_eq!(
            Board::from_str(&[
                "22 13 17 11  0",
                " 8  2 23  4 24",
                "21  9 14 16  7",
                " 6 10  3 18  5",
                " 1 12 20 15 19"
            ]),
            Board::new(hashmap! {
                22 => (0, 0),
                13 => (0, 1),
                17 => (0, 2),
                11 => (0, 3),
                0 => (0, 4),
                8 => (1, 0),
                2 => (1, 1),
                23 => (1, 2),
                4 => (1, 3),
                24 => (1, 4),
                21 => (2, 0),
                9 => (2, 1),
                14 => (2, 2),
                16 => (2, 3),
                7 => (2, 4),
                6 => (3, 0),
                10 => (3, 1),
                3 => (3, 2),
                18 => (3, 3),
                5 => (3, 4),
                1 => (4, 0),
                12 => (4, 1),
                20 => (4, 2),
                15 => (4, 3),
                19 => (4, 4)
            })
        )
    }

    #[test]
    fn test_play_game() {
        assert_eq!(
            play_game(
                vec![
                    7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18,
                    20, 8, 19, 3, 26, 1
                ],
                &mut vec![
                    Board::from_str(&[
                        "22 13 17 11  0",
                        " 8  2 23  4 24",
                        "21  9 14 16  7",
                        " 6 10  3 18  5",
                        " 1 12 20 15 19"
                    ]),
                    Board::from_str(&[
                        " 3 15  0  2 22",
                        " 9 18 13 17  5",
                        "19  8  7 25 23",
                        "20 11 10 24  4",
                        "14 21 16 12  6"
                    ]),
                    Board::from_str(&[
                        "14 21 17 24  4",
                        "10 16 15  9 19",
                        "18  8 23 26 20",
                        "22 11 13  6  5",
                        " 2  0 12  3  7"
                    ])
                ]
            ),
            4512
        )
    }
}
