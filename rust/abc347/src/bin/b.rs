use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {
        s: String
    }

    let mut set = HashSet::new();

    for i in 0..s.len() {
        for j in i..s.len() {
            set.insert(s.chars().skip(i).take(j - i + 1).collect::<String>());
        }
    }

    println!("{}", set.len());
}
