use proconio::*;

fn main() {
    input! {
        h: usize, w: usize, x: usize, y: usize,
        s: [String; h],
    }
    let mut count = 1;

    let mut left = String::new();
    for (i, char) in s[x - 1].chars().enumerate() {
        if i < y - 1 {
            left.push(char);
            continue;
        }
        if i > y - 1 {
            if !char.eq(&'.') {
                break;
            }
            count += 1;
        }
    }
    count += left
        .chars()
        .rev()
        .collect::<String>()
        .find('#')
        .unwrap_or(left.len());

    let mut up = String::new();
    for (i, row) in s.iter().enumerate() {
        let cur_char = row.chars().nth(y - 1).unwrap_or(' ');
        if i < x - 1 {
            up.push(cur_char);
            continue;
        }
        if i > x - 1 {
            if !cur_char.eq(&'.') {
                break;
            }
            count += 1;
        }
    }
    count += up
        .chars()
        .rev()
        .collect::<String>()
        .find('#')
        .unwrap_or(up.len());

    println!("{}", count);
}
