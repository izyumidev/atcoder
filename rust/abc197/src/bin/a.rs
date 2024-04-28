use proconio::*;

fn main() {
    input! {
        s: String
    }
    print!(
        "{}{}",
        s.chars().skip(1).collect::<String>(),
        s.chars().nth(0).unwrap()
    );
}
