use proconio::*;

fn main() {
    input! {
        n: usize
    }

    let mut max = 1;

    for i in 1..=n {
        let cube = i.pow(3);
        if cube > n {
            break;
        }
        if is_palindrome(cube) {
            max = cube;
        }
    }

    println!("{}", max);
}

fn is_palindrome(s: usize) -> bool {
    let s_str = s.to_string();
    s_str.chars().rev().collect::<String>() == s_str.chars().collect::<String>()
}
