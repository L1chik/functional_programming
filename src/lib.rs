use std::cmp::max;
use std::collections::hash_map::DefaultHasher;
use num_bigint::{BigUint, ToBigUint};
use num_traits::pow;
use rand::Rng;
// use num::signum;
// use num_bigint::RandBigInt;

use std::hash::{Hash, Hasher};

pub fn ex_3(v1: i32, v2: i32) -> i32 {
    max(v1, v2)
}

pub fn ex_4() -> BigUint {
    let num: BigUint = 2.to_biguint().unwrap();
    pow(num, 1024)
}

// pub fn ex_6() -> () {
//     let mut hasher = DefaultHasher::new();
//     let mut rng = rand::thread_rng();
//     let n1: i32 = rng.gen();
//     let n2 = n1.to_biguint().unwrap();
//
//     Hash::hash(&n2, &mut hasher);
//     println!("Hash is {:x}", hasher.finish());
// }

pub fn ex_7() -> (char, char) {
    let txt = "hello world!";
    let first = txt.chars().next().unwrap();
    let last = txt.chars().last().unwrap();
    (first, last)
}

pub fn signum(num: i32) -> i8 {
    match num {
        num if num > 0 => 1,
        0 => 0,
        _ => -1,
    }
}

pub fn ex_11() {
    for it in (0..11).rev() {
        print!("{}, ", it);
    }
}

pub fn ex_12(num: i32) -> i32 {
    print!("{}, ", num);

    return match num {
        num if num > 0 => ex_12(num - 1),
        _ => -1,
    };
}

pub fn ex_13(txt: &str) -> u128 {
    let mut answer: u128 = 1;
    let mut vec = txt.chars().map(|x| x as u128).collect::<Vec<_>>();

    for el in vec.iter_mut() {
        answer *= *el;
    }

    answer
}

pub fn ex_14(txt: &str) -> u128 {
    let answer: u128 = txt.chars()
        .map(|x| x as u128)
        .reduce(|x, y| x * y)
        .unwrap();

    answer
}

pub fn ex_17(x: f64, y: f64) -> f64 {
    println!("{}", y);
    return match y {
        0. => 1.,
        y if y.is_positive() & (y % 2. == 0.) => 1./(x-y),
        y if y.is_positive() & !(y % 2. == 0.) => x * ex_17(x,y - 1.),
        _ => {
            let num = ex_17(x, y / 2.);
            num * num
        }
    };
}