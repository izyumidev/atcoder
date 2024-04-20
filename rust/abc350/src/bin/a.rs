use proconio::*;

fn main() {
    input! {
        s: String
    }
    let num = s[3..6].parse::<i32>();
    if num.is_err() {
        print!("No");
        return;
    }
    let num = num.unwrap();
    if s.starts_with("ABC") && s.len() == 6 && num < 350 && num > 0 && num != 316 {
        println!("Yes")
    } else {
        print!("No")
    }
}
