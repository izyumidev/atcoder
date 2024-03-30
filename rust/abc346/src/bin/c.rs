use proconio::*;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }

    let mut res = Vec::new();

    for i in 1..=k {
        if !a.contains(&i) {
            res.push(i);
        }
    }

    println!("{}", res.iter().sum::<usize>());
}
