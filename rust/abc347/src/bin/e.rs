use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {
        n: usize, q: usize,
        x: [usize; q],
    }

    let mut a = vec![0; n];
    let mut s = HashSet::new();

    for i in x {
        if s.contains(&i) {
            s.remove(&i);
        } else {
            s.insert(i);
        }
        let len = s.len();
        for j in 1..=n {
            if s.contains(&j) {
                a[j - 1] += len;
            }
        }
    }

    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
