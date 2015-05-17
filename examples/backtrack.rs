#![deny(warnings)]

extern crate forkallcc;

use std::borrow::ToOwned;
use forkallcc::{Cont, call_cc};

struct Backtrack {
    checkpoint: Option<Cont<bool>>,
}

impl Backtrack {
    fn new() -> Backtrack {
        Backtrack {
            checkpoint: None,
        }
    }

    fn fail(&self) -> ! {
        match self.checkpoint {
            Some(ref k) => k.invoke(false),
            None => panic!("Nothing to be done."),
        }
    }

    fn guess(&mut self) -> bool {
        call_cc(|k| {
            self.checkpoint = Some(k);
            true
        })
    }

    fn guess_from<I>(&mut self, it: I) -> <I as Iterator>::Item
        where I: Iterator
    {
        for i in it {
            if self.guess() {
                return i;
            }
        }
        self.fail();
    }
}

fn factor(n: u32) -> (u32, u32) {
    let mut bt = Backtrack::new();
    let i = bt.guess_from(2..100);
    let j = bt.guess_from(2..100);

    if i*j != n {
        bt.fail();
    }

    (i, j)
}

fn factor_391() {
    const N: u32 = 391;
    let (i, j) = factor(N);
    println!("{} * {} = {}\n", i, j, N);

    assert!(i != 1);
    assert!(j != 1);
    assert_eq!(i * j, N);
}

fn solve_maze() {
    const MAZE_SIZE: i32 = 15;

    let mut maze = "\
        X ------------+\n\
        |       |     |\n\
        |--+  | |   | |\n\
        |  |  | | --+ |\n\
        |     |     | |\n\
        |-+---+--+- | |\n\
        | |      |    |\n\
        | | | ---+-+- |\n\
        |   |      |  |\n\
        | +-+-+--|    |\n\
        | |   |  |--- |\n\
        |     |       |\n\
        |--- -+-------|\n\
        |              \n\
        +------------- \n".to_owned().into_bytes();

    let mut bt = Backtrack::new();
    let mut x = 0;
    let mut y = 0;

    while (x != MAZE_SIZE-1) || (y != MAZE_SIZE-1) {
             if bt.guess() { x += 1; }
        else if bt.guess() { x -= 1; }
        else if bt.guess() { y += 1; }
        else               { y -= 1; }

        if (x < 0) || (x >= MAZE_SIZE) || (y < 0) || (y >= MAZE_SIZE) {
            bt.fail();
        }

        let i = (y*(MAZE_SIZE+1) + x) as usize;
        if maze[i] != b' ' {
            bt.fail();
        }

        maze[i] = b'X';
    }

    for c in String::from_utf8(maze).unwrap().chars() {
        if c == 'X' {
            print!("\x1B[1;32mX\x1B[0m");
        } else {
            print!("{}", c);
        }
    }
}

fn main() {
    factor_391();
    solve_maze();
}
