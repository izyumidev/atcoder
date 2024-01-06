use proconio::*;

fn main() {
    input! {
        k: usize, g: usize, m: usize,
    }
    let (mut gc, mut mc) = (0, 0);
    for _ in 0..k {
        if gc == g {
            gc = 0;
        } else if mc == 0 {
            mc = m;
        } else {
            let left_in_glass = g - gc;
            if left_in_glass < mc {
                mc -= left_in_glass;
                gc = g;
            } else {
                gc += mc;
                mc = 0;
            }
        }
    }
    println!("{} {}", gc, mc);
}
