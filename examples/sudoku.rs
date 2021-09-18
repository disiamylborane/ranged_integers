//! A Sudoku problem from RosettaCode (https://rosettacode.org/wiki/Sudoku#Rust)
//! rewritten with the use of ranged_integers library

#![allow(incomplete_features)]
#![feature(adt_const_params,generic_const_exprs)]

use ranged_integers::*;

pub const ROW_SIZE : usize = 9;

pub type Val = Ranged<0, 9>;
pub type RowIndex = Ranged<0, {({ROW_SIZE - 1}) as _}>;
pub type Sudoku = [[Val; 9]; 9];

pub fn is_valid(val: Val, x: RowIndex, y: RowIndex, sudoku_ar: &Sudoku) -> bool {
    r!(0..9).into_iter().all( |i|
        sudoku_ar[x][i] != val && 
        sudoku_ar[i][y] != val && {
            let r3 = || r!(0..3);
            r3().into_iter().all(|i| r3().into_iter().all(|j| 
                sudoku_ar[x / r!(3) * r!(3) + i][y / r!(3) * r!(3) + j] != val
            ))
        }
    )
}

pub fn place_number(pos: Ranged<0, 80>, sudoku_ar: &mut Sudoku) -> bool {
    pos.iter_up()
        .find_map(|p| {
                let (x, y) = (p % r!(9), p / r!(9));
                if sudoku_ar[x][y] == r!(0) {Some((x,y))} else {None}
            })
        .map_or(true, |(x, y)| {
            for n in r!(1..10) {
                if is_valid(n.expand(), x, y, sudoku_ar) {
                    sudoku_ar[x][y] = n.expand();
                    let next = if let Some(next) = (pos + r!(1)).fit() {next} else {return true};
                    if place_number(next,sudoku_ar) {
                        return true;
                    }
                    sudoku_ar[x][y] = r!([]0);
                }
            }
            false
        })
}

pub fn pretty_print(sudoku_ar: Sudoku) {
    let line_sep = "------+-------+------";
    println!("{}", line_sep);
    for (i, row) in sudoku_ar.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            print!("{} ", val);
            if j == 2 || j == 5 {
                print!("| ");
            }
        }
        println!("");
        if i % 3 == 2 {
            println!("{}", line_sep);
        }
    }
}

fn solve(sudoku_ar: &mut Sudoku)->bool {
    place_number(r!([] 0), sudoku_ar)
}

macro_rules! rangedarr {
    ($($e:literal),* $(,)?) => {  [$( r!([] $e) ),*]  };
} 

fn main() {
    let mut sudoku_ar: Sudoku = [
        rangedarr![8, 5, 0, 0, 0, 2, 4, 0, 0],
        rangedarr![7, 2, 0, 0, 0, 0, 0, 0, 9],
        rangedarr![0, 0, 4, 0, 0, 0, 0, 0, 0],
        rangedarr![0, 0, 0, 1, 0, 7, 0, 0, 2],
        rangedarr![3, 0, 5, 0, 0, 0, 9, 0, 0],
        rangedarr![0, 4, 0, 0, 0, 0, 0, 0, 0],
        rangedarr![0, 0, 0, 0, 8, 0, 0, 7, 0],
        rangedarr![0, 1, 7, 0, 0, 0, 0, 0, 0],
        rangedarr![0, 0, 0, 0, 3, 6, 0, 4, 0],
    ];

    if solve(&mut sudoku_ar) {
        pretty_print(sudoku_ar);
    }
    else {
        println!("Unsolvable");
    }
}
