fn main() {
    let (n, m) = {
        let mut v: Vec<u32> = Vec::new();
        for i in input().split_whitespace() {
            v.push(i.parse().unwrap());
        }
        (v[0], v[1])
    };

    let mut cards: Vec<u32> = Vec::new();
    for i in input().split_whitespace() {
        cards.push(i.parse().unwrap());
    }

    // let start = std::time::Instant::now();
    let mut max: u32 = 0;
    for i in 0..n as usize {
        for j in i + 1..n as usize {
            for k in j + 1..n as usize {
                let sum = cards[i] + cards[j] + cards[k];
                if sum <= m && max < sum {
                    max = sum;
                }
            }
        }
    }

    println!("{}", max);
    // println!("{:?}", start.elapsed());
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}