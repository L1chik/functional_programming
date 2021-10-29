use std::cmp::max;
use num_bigint::{BigUint, ToBigUint};
use num_traits::pow;
use core::hash;
// use rand::Rng;

mod lib;

fn main() {
    //1

    // №2
    println!("Задание 2 {:♥<1$}", "", 3);

    // №3 - 10 max 2 возвращает большее из двух чисел
    println!("Задание 3 {}", lib::ex_3(10, 2));

    // №4
    println!("Задание 4 {:?}", lib::ex_4());

    // №5 use rand::Rng;
    // println!("{}", rng.gen())

    // №6
    // lib::ex_6();

    // №7
    println!("Задание 7 {:?}", lib::ex_7());

    // №8

    // №9
    let num: i32 = 9;
    println!("Задание 9: сигнум числа {1} -> {0},", lib::signum(num), num);

    // №11
    println!("Задание 11:");
    lib::ex_11();
    println!();

    // №12
    println!("Задание 12 {:?}", lib::ex_12(20));

    // №13
    println!("Задание 13: {}", lib::ex_13("Hello"));

    // №14
    println!("Задание 14: {:?}", lib::ex_14("Hello"));

    // №17
    println!("Задание 17: {:?}", lib::ex_17(2.0, 1.0));
}

