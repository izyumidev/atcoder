use proconio::*;

fn main() {
    input! {
        a: usize, b: usize, c: usize
    }

    for x in 0..2u64.pow(60) {
        for y in 0..2u64.pow(60) {
            if popcount(x, a) && popcount(y, b) && x ^ y == c as u64 {
                println!("{} {}", x, y);
                break;
            }
        }
    }
}

fn popcount(x: u64, n: usize) -> bool {
    x.count_ones() == n as u32
}
