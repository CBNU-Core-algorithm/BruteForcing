fn main() {
    let (min, max) = {
        let mut v: Vec<u64> = Vec::new();
        for i in input().split_whitespace() {
            v.push(i.parse().unwrap());
        }
        (v[0], v[1])
    };
    let mut res: u64 = 0;

    gold_minsu(&mut res, 4, max, min);
    gold_minsu(&mut res, 7, max, min);

    println!("{}", res);
}

fn gold_minsu(mut res: &mut u64, n: u64, max: u64, min: u64) {
    if n >= min && n <= max {
        *res += 1;
    } else {
        return;
    }

    gold_minsu(res, n * 10 + 4, max, min);
    gold_minsu(res, n * 10 + 7, max, min);
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
