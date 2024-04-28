use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [String; n],
        b: [String; n],
    }
    for i in 0..n {
        for j in 0..n {
            if a[i].chars().nth(j) != b[i].chars().nth(j) {
                println!("{} {}", i + 1, j + 1);
                break;
            }
        }
    }
}
