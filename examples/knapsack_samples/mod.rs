// This file is based on samples from
// https://people.sc.fsu.edu/~jburkardt/datasets/knapsack_01/knapsack_01.html
// and is disttributed under the GNU LGPL v3 license (see gnu_lgpl.txt).

#![allow(non_upper_case_globals)]
use std::collections::HashSet;

use super::Item;

const fn i(weight: u32, price: u32) -> Item {
    Item { weight, price }
}

// P01

pub const capacity1: u32 = 165;

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

// P02

pub const capacity2: u32 = 26;

pub fn items2() -> Vec<Item> {
    vec![i(12, 24), i(7, 13), i(11, 23), i(8, 15), i(9, 16)]
}

pub fn expected2() -> HashSet<Item> {
    let items = items2();
    HashSet::from([items[1].clone(), items[2].clone(), items[3].clone()])
}

// P03

pub const capacity3: u32 = 190;

pub fn items3() -> Vec<Item> {
    vec![
        i(56, 50),
        i(59, 50),
        i(80, 64),
        i(64, 46),
        i(75, 50),
        i(17, 5),
    ]
}

pub fn expected3() -> HashSet<Item> {
    let items = items3();
    HashSet::from([items[0].clone(), items[1].clone(), items[4].clone()])
}

// P04

pub const capacity4: u32 = 50;

pub fn items4() -> Vec<Item> {
    vec![
        i(31, 70),
        i(10, 20),
        i(20, 39),
        i(19, 37),
        i(4, 7),
        i(3, 5),
        i(6, 10),
    ]
}

pub fn expected4() -> HashSet<Item> {
    let items = items4();
    HashSet::from([items[0].clone(), items[3].clone()])
}

// P05

pub const capacity5: u32 = 104;

pub fn items5() -> Vec<Item> {
    vec![
        i(25, 350),
        i(35, 400),
        i(45, 450),
        i(5, 20),
        i(25, 70),
        i(3, 8),
        i(2, 5),
        i(2, 5),
    ]
}

pub fn expected5() -> HashSet<Item> {
    let items = items5();
    HashSet::from([
        items[0].clone(),
        items[2].clone(),
        items[3].clone(),
        items[4].clone(),
        items[6].clone(),
        items[7].clone(),
    ])
}

// P06

pub const capacity6: u32 = 170;

pub fn items6() -> Vec<Item> {
    vec![
        i(41, 442),
        i(50, 525),
        i(49, 511),
        i(59, 593),
        i(55, 546),
        i(57, 564),
        i(60, 617),
    ]
}

pub fn expected6() -> HashSet<Item> {
    let items = items6();
    HashSet::from([items[1].clone(), items[3].clone(), items[6].clone()])
}

// P07

pub const capacity7: u32 = 750;

pub fn items7() -> Vec<Item> {
    vec![
        i(70, 135),
        i(73, 139),
        i(77, 149),
        i(80, 150),
        i(82, 156),
        i(87, 163),
        i(90, 173),
        i(94, 184),
        i(98, 192),
        i(106, 201),
        i(110, 210),
        i(113, 214),
        i(115, 221),
        i(118, 229),
        i(120, 240),
    ]
}

pub fn expected7() -> HashSet<Item> {
    let items = items7();
    HashSet::from([
        items[0].clone(),
        items[2].clone(),
        items[4].clone(),
        items[6].clone(),
        items[7].clone(),
        items[8].clone(),
        items[13].clone(),
        items[14].clone(),
    ])
}

// P08

pub const capacity8: u32 = 6404180;

pub fn items8() -> Vec<Item> {
    vec![
        i(382745, 825594),
        i(799601, 1677009),
        i(909247, 1676628),
        i(729069, 1523970),
        i(467902, 943972),
        i(44328, 97426),
        i(34610, 69666),
        i(698150, 1296457),
        i(823460, 1679693),
        i(903959, 1902996),
        i(853665, 1844992),
        i(551830, 1049289),
        i(610856, 1252836),
        i(670702, 1319836),
        i(488960, 953277),
        i(951111, 2067538),
        i(323046, 675367),
        i(446298, 853655),
        i(931161, 1826027),
        i(31385, 65731),
        i(496951, 901489),
        i(264724, 577243),
        i(224916, 466257),
        i(169684, 369261),
    ]
}

pub fn expected8() -> HashSet<Item> {
    let items = items8();
    HashSet::from([
        items[0].clone(),
        items[1].clone(),
        items[3].clone(),
        items[4].clone(),
        items[5].clone(),
        items[9].clone(),
        items[10].clone(),
        items[12].clone(),
        items[15].clone(),
        items[21].clone(),
        items[22].clone(),
        items[23].clone(),
    ])
}
