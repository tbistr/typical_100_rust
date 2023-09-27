use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.push(l);

    let mut left = 0;
    let mut right = usize::MAX;

    while 1 < right - left {
        let m = (left + right) / 2;
        let mut cnt = 0;
        let mut pre = 0;
        for i in &a {
            if m <= i - pre {
                cnt += 1;
                pre = *i;
            }
        }

        if k < cnt {
            left = m;
        } else {
            right = m;
        }
    }
    println!("{}", left);
}
