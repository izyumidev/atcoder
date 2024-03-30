use proconio::*;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }

    let mut res = vec![];

    for num in a {
        if num % k == 0 {
            res.push(num / k);
        }
    }

    println!(
        "{}",
        res.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
