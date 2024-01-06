use proconio::*;

fn main() {
    input! {
        n: usize, s: usize, k: usize,
        pq: [(usize, usize); n],
    }
    let mut sum = 0;
    for (p, q) in pq {
        sum += p * q;
    }
    if sum >= s {
        println!("{}", sum);
        return;
    }
    println!("{}", sum + k);
}
