use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut seq = Vec::new();
    for op in a {
        seq.push(op);
        'inner: loop {
            if seq.len() <= 1 || seq[seq.len() - 1] != seq[seq.len() - 2] {
                break 'inner;
            }
            let last = seq.pop().unwrap() + 1;
            seq.pop();
            seq.push(last);
        }
    }

    println!("{}", seq.len());
}
