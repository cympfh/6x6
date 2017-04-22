#![allow(unused_imports)]
use std::env;
use std::io::{ self, Write };
use std::str::FromStr;
use std::cmp::{ min, max };
use std::collections::{ BinaryHeap, VecDeque };

extern crate rand;
use rand::{random, Open01};

macro_rules! trace {
    ($var:expr) => ({
        let _ = writeln!(&mut std::io::stderr(), ">>> {} = {:?}", stringify!($var), $var);
    })
}

fn usage() {
    println!(r#"6x6

Usage:
    6x6 [COMMAND]

COMMAND:

    solve
    put
    check
"#);

}

const M: usize = 6;  // the size of the fields
const N: usize = 100; // the num of Monte Carlo

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell { Empty, O, X }
use Cell::{ Empty, O, X };
impl Cell {
    fn from_char(c: char) -> Cell {
        if c == 'o' {
            O
        } else if c == 'x' {
            X
        } else {
            Empty
        }
    }
}

fn read_board(sc: &mut Scanner) -> Vec<Vec<Cell>> {
    let fs = (0..6).map(|_|
                        (0..6).map(|_|
                                   Cell::from_char(sc.get_char())
                                  ).collect()).collect();
    fs
}

fn note2pos(note: String) -> (usize, usize) {
    let cs = note.chars().collect::<Vec<_>>();
    let i = (cs[1] as u8 - '1' as u8) as usize;
    let j = (cs[0] as u8 - 'a' as u8) as usize;
    (i, j)
}

fn pos2note(i: usize, j: usize) -> String {
    let alphabet = ('a' as u8 + j as u8) as char;
    let digit = ('1' as u8 + i as u8) as char;
    format!("{}{}", alphabet, digit)
}

fn opposite(c: Cell) -> Cell {
    if c == O {
        X
    } else if c == X {
        O
    } else {
        Empty
    }
}

fn in_range(i: i32, j: i32) -> bool {
    i >= 0 && i < M as i32 && j >= 0 && j < M as i32
}

fn puttable(fs: &Vec<Vec<Cell>>, next: Cell) -> Vec<(usize, usize)> {

    let dx: [i32; 8] = [-1, -1, -1, 0, 1, 1, 1, 0];
    let dy: [i32; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];

    let mut r = vec![];
    let prev = opposite(next);

    for i in 0..6 {
        for j in 0..6 {

            if fs[i][j] != Empty { continue }

            for k in 0..8 {
                let mut nb_opposite = 0;
                let mut _i = i as i32 + dx[k];
                let mut _j = j as i32 + dy[k];
                let mut ok = false;

                while in_range(_i, _j) {
                    let f = fs[_i as usize][_j as usize];
                    if f == prev {
                        nb_opposite += 1;
                    } else if f == next {
                        if nb_opposite > 0 { // success
                            ok = true;
                        }
                        // else failed
                        break;
                    } else {
                        break; // failed
                    }
                    _i = _i as i32 + dx[k];
                    _j = _j as i32 + dy[k];
                }

                if ok {
                    r.push((i, j));
                    break;
                }
            }
        }
    }
    r
}

fn put(fs: &mut Vec<Vec<Cell>>, next: Cell, i: usize, j: usize) -> bool {

    let mut ok = false;

    let dx: [i32; 8] = [-1, -1, -1, 0, 1, 1, 1, 0];
    let dy: [i32; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];
    let prev = opposite(next);
    fs[i][j] = next;

    for k in 0..8 {
        let mut ps = vec![];
        let mut _i = i as i32 + dx[k];
        let mut _j = j as i32 + dy[k];

        while in_range(_i, _j) {
            let f = fs[_i as usize][_j as usize];
            if f == prev {
                ps.push((_i, _j));
            } else if f == next {
                for (__i, __j) in ps {
                    fs[__i as usize][__j as usize] = next;
                    ok = true;
                }
                break;
            } else {
                break;
            }
            _i = _i as i32 + dx[k];
            _j = _j as i32 + dy[k];
        }
    }

    ok
}

fn majority(fs: &Vec<Vec<Cell>>) -> Cell {
    let mut k = 0;
    for i in 0..6 {
        for j in 0..6 {
            if fs[i][j] == O {
                k += 1;
            } else if fs[i][j] == X {
                k -= 1;
            }
        }
    }
    if k > 0 { O } else { X }
}

fn end(fs: &Vec<Vec<Cell>>) -> Cell {
    let mut nb_o = 0;
    let mut nb_x = 0;
    for i in 0..6 {
        for j in 0..6 {
            if fs[i][j] == O {
                nb_o += 1
            } else if fs[i][j] == X {
                nb_x += 1
            }
        }
    }

    if (nb_x + nb_o == 0) || (puttable(&fs, O).len() == 0 && puttable(&fs, X).len() == 0) {
        Empty
    } else {
        if nb_o > nb_x {
            O
        } else {
            X
        }
    }
}

fn display(fs: &Vec<Vec<Cell>>) {
    for i in 0..6 {
        for j in 0..6 {
            let f = fs[i][j];
            print!("{}", if f == Empty { '.' } else if f == O { 'o' } else { 'x' });
        }
        println!("");
    }
}

fn random_choice(hands: &Vec<(usize, usize)>) -> (usize, usize) {

    fn is_corner(i: usize, j: usize) -> bool {
        (i == 0 && j == 0) ||
        (i == 0 && j == M - 1) ||
        (i == M - 1 && j == 0) ||
        (i == M - 1 && j == M - 1)
    }

    let n = hands.len();
    let ps = hands.iter().map(|&hand| {
        let (i, j) = hand;
        if is_corner(i, j) {
            340.0
        } else {
            1.0
        }
    }).collect::<Vec<_>>();
    let psum = ps.iter().fold(0.0, |ac, &x| ac + x);

    let Open01(mut q) = random::<Open01<f32>>();
    q *= psum;

    for i in 0..n {
        if q < ps[i] {
            return hands[i]
        }
        q -= ps[i]
    }
    hands[0]
}

fn random_play(mut fs: &mut Vec<Vec<Cell>>, next: Cell) -> Cell {

    let mut cur = next;

    let mut nb_pass = 0;

    loop {
        let winner = end(&fs);
        if winner != Empty {
            // trace!(winner);
            // display(&fs);
            return winner
        }
        let hands = puttable(&fs, cur);
        let n = hands.len();

        if n > 0 {
            let (i, j) = random_choice(&hands);
            put(&mut fs, cur, i, j);
        } else {
            nb_pass += 1;
            if nb_pass > 6 {
                return majority(&fs);
            }
        }
        cur = opposite(cur);
    }
}

fn solve(mut fs: &mut Vec<Vec<Cell>>, next: Cell) {

    let prev = opposite(next);

    // (Strength, Potision)
    let mut result: Vec<(i32, (usize, usize))> = vec![];

    // Monte Carlo
    for (i, j) in puttable(&fs, next) {
        let mut strength = 0;
        for _ in 0..N {
            let mut gs = fs.clone();
            put(&mut gs, next, i, j);
            if random_play(&mut gs, prev) == next {
                strength += 1;
            }
        }
        trace!((strength, pos2note(i, j)));
        result.push((strength, (i, j)));
    }

    if result.len() == 0 {
        println!("pass");
    } else {
        result.sort_by(|a, b| b.0.cmp(&a.0));
        let (i, j) = result[0].1;
        println!("{}", pos2note(i, j));
        put(&mut fs, next, i, j);
        display(&fs);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sc = Scanner::new();

    if args.len() < 2 {

        usage();

    } else if args[1] == "solve" {

        let next = Cell::from_char(sc.get_char());
        let mut fs = read_board(&mut sc);
        solve(&mut fs, next)

    } else if args[1] == "put" {

        let next = Cell::from_char(sc.get_char());
        let (i, j) = note2pos(sc.cin::<String>());
        let mut fs = read_board(&mut sc);
        let result = put(&mut fs, next, i, j);
        if result {
            println!("ok");
            display(&fs);
        } else {
            println!("invalid");
        }

    } else if args[1] == "check" {

        let fs = read_board(&mut sc);
        let result = end(&fs);
        if result == Empty {
            println!("yet");
        } else {
            println!("end");
            println!("{}", if result == O { 'o' } else { 'x' });
        }

    } else {
        println!("unknown command: {}", args[1]);
        usage();
    }
}

#[allow(dead_code)]
struct Scanner { stdin: io::Stdin, buffer: VecDeque<String>, }
#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner { Scanner { stdin: io::stdin(), buffer: VecDeque::new() } }
    fn reserve(&mut self) {
        while self.buffer.len() == 0 {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        self.reserve();
        match self.buffer.pop_front().unwrap().parse::<T>() {
            Ok(a) => a,
            Err(_) => panic!("parse err")
        }
    }
    fn get_char(&mut self) -> char {
        self.reserve();
        let head = self.buffer[0].chars().nth(0).unwrap();
        let tail = String::from( &self.buffer[0][1..] );
        if tail.len()>0 { self.buffer[0]=tail } else { self.buffer.pop_front(); }
        head
    }
}
