// This file is based on samples from
// https://people.sc.fsu.edu/~jburkardt/datasets/knapsack_01/knapsack_01.html
// and is disttributed under the GNU LGPL v3 license (see gnu_lgpl.txt).

#![allow(non_upper_case_globals)]
use std::collections::HashSet;

use super::Item;

const fn i(weight: u32, price: u32) -> Item {
    Item { weight, price }
}

pub static capacity1: u32 = 165;

pub fn items1() -> Vec<Item> {
    vec![
        i(23, 92),
        i(31, 57),
        i(29, 49),
        i(44, 68),
        i(53, 60),
        i(38, 43),
        i(63, 67),
        i(85, 84),
        i(89, 87),
        i(82, 72),
    ]
}

pub fn expected1() -> HashSet<Item> {
    let items = items1();
    HashSet::from([
        items[0].clone(),
        items[1].clone(),
        items[2].clone(),
        items[3].clone(),
        items[5].clone(),
    ])
}
