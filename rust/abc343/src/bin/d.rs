use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {
        n: usize, t: usize,
        ab: [(usize, usize); t]
    }

    let mut scores = vec![0; n];

    for (a, b) in ab.iter() {
        scores[*a - 1] += *b;
        let set: HashSet<usize> = HashSet::from_iter(scores.iter().cloned());
        println!("{}", set.len());
    }
}
