fn main() {
    let n = input().parse().unwrap();

    let mut data: Vec<(u32, u32, u32)> = Vec::new();
    for _ in 0..n {
        data.push({
            let mut v: Vec<u32> = Vec::new();
            for val in input().split_whitespace() {
                v.push(val.parse().unwrap());
            }
            (v[0], v[1], 1)
        });
    }

    let data_len = data.len();
    for i in 0..data_len {
        for j in 0..data_len {
            if data[j].0 > data[i].0 && data[j].1 > data[i].1 {
                data[i].2 += 1;
            }
        }
    }

    for (_, _, rank) in data.into_iter() {
        print!("{} ", rank);
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
