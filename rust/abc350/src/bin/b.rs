use proconio::*;

fn main() {
    input! {
        n: usize, q: usize,
        t: [usize; q]
    }
    let mut teeth = vec![true; n];
    for a in t {
        teeth[a - 1] = !teeth[a - 1];
    }
    println!("{}", teeth.iter().filter(|x| **x).count())
}
