use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut res = Vec::new();

    a.windows(2).for_each(|x| {
        res.push(x[0] * x[1]);
    });

    println!(
        "{}",
        res.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
