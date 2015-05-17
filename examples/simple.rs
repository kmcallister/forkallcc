#![deny(warnings)]

extern crate forkallcc;

use forkallcc::{Cont, call_cc};

#[allow(unreachable_code)]
fn early_return() {
    let r = call_cc(|k| {
        println!("early_return: closure called");
        k.invoke(1);
        println!("early_return: continuation called");
        0
    });
    println!("early_return: closure returns {}\n", r);
    assert_eq!(r, 1);
}

fn double_return() {
    let mut saved_k: Option<Cont<i32>> = None;
    let r = call_cc(|k| {
        println!("double_return: closure called");
        saved_k = Some(k);
        0
    });
    println!("double_return: closure returns {}", r);
    match saved_k {
        None => {
            assert_eq!(r, 1);
            println!("double_return: no saved continuation\n");
        }
        Some(k) => {
            assert_eq!(r, 0);
            k.invoke(1);
        }
    }
}

fn main() {
    early_return();
    double_return();
}
