use std::collections::HashMap;

fn main() {
    loop {
        let (min, max) = {
            let mut v: Vec<u64> = Vec::new();
            for i in input().split_whitespace() {
                v.push(i.parse().unwrap());
            }
            (v[0], v[1])
        };
        let mut res = 0;

        for i in min..max + 1 {
            if is_cursed(i) {
                res += 1;
            }
        }

        println!("{}", res);
    }
}

fn is_cursed(n: u64) -> bool {
    let mut map: HashMap<String, u8> = HashMap::new();
    for ch in n.to_string().chars() {
        *map.entry(ch.to_string()).or_insert(0) += 1;
        let doubled = map[&ch.to_string()];
        if map[&ch.to_string()] == 2 {
            return false;
        }
    }
    return true;
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
