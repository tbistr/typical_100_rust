use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in (0..1 << n).rev() {
        let mut cnt = 0;
        let mut result = String::new();
        for j in (0..n).rev() {
            if i & 1 << j != 0 {
                cnt += 1;
                result.push('(');
            } else {
                cnt -= 1;
                result.push(')');
            }
            if cnt < 0 {
                break;
            }
        }
        if cnt == 0 {
            println!("{}", result);
        }
    }
}
