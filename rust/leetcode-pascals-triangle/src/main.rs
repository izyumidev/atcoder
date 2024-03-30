struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1]];
        match num_rows {
            1 => res,
            n => {
                res.push(vec![1, 1]);
                for i in 3..=n {
                    let mut cur = Vec::new();
                    let last_row = &res.last().unwrap();
                    cur.push(1);
                    for x in 0..last_row.len() - 1 {
                        cur.push(last_row[x] + last_row[x + 1]);
                    }
                    cur.push(1);
                    res.push(cur);
                }
                res
            }
        }
    }
}

fn main() {}
