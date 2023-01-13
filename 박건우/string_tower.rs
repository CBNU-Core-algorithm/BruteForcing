use std::fmt::{Debug, Formatter};

struct Attribute {
    prefix: Vec<String>,
    suffix: Vec<String>,
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n    prefix: {:?},\n    suffix: {:?}\n}}\n", self.prefix, self.suffix)
    }

}

fn main() {
    let (n, _) = {
        let mut v: Vec<u64> = Vec::new();
        for i in input().split_whitespace() {
            v.push(i.parse().unwrap());
        }
        (v[0], v[1])
    };

    let (mut prev, mut prev_attr) = (String::new(), Attribute { prefix: Vec::new(), suffix: Vec::new() });
    for i in 0..n {
        if i == 0 {
            prev = input();
            prev_attr = get_attr(&prev);
        }
        else {
            let mut flag = false;
            let cur = input();
            let cur_attr = get_attr(&cur);
            // println!("prev: {:?}\nprev_attr: {:?}", prev, prev_attr);
            // println!("cur: {:?}\ncur_attr: {:?}", cur, cur_attr);
            for pre in prev_attr.prefix.clone().into_iter() {
                for suf in cur_attr.suffix.clone().into_iter() {
                    if pre == suf {
                        flag =true;
                        // println!("{}", pre == suf);
                        // println!("match {} {}", pre, suf);
                    }
                }
            }
            if !flag {
            for pre in cur_attr.prefix.clone().into_iter() {
                for suf in prev_attr.suffix.clone().into_iter() {
                    if pre == suf {
                        flag = true;
                        // println!("{}", pre == suf);
                        // println!("match {} {}", pre, suf);
                    }
                }
            }
                }
            if !flag {
                println!("0");
                return;
            }
            prev = cur;
            prev_attr = cur_attr;
        }
    }
    println!("1");
}
fn get_attr(s: &String) -> Attribute {
    let mut prefix: Vec<String> = Vec::new();
    let mut suffix: Vec<String> = Vec::new();
    for i in 0..s.len() {
        prefix.push(s[..i+1].to_string());
        suffix.push(s[i..].to_string());
    }
    Attribute { prefix, suffix }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
