use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut count = 0;
    let mut moves: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        if a[i] == i + 1 {
            continue;
        }
        for j in i..n {
            if a[j] == i + 1 {
                let tmp = a[j];
                a[j] = a[i];
                a[j] = tmp;
                count += 1;
                moves.push((i, j));
            }
        }
    }
    println!("{}", count);
    for (x, y) in moves {
        println!("{} {}", x, y);
    }
}
