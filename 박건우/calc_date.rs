fn main() {
    let (mut e,mut s,mut m) = {
        //parse trhee number from input
        let mut v: Vec<i32> = Vec::new();
        for i in input().split_whitespace() {
            v.push(i.parse().unwrap());
        }
        (v[0], v[1], v[2])
    };
    let start = std::time::Instant::now();
    let mut year = 1;
    loop {
        if (year-e)%15 == 0 && (year-s)%28 == 0&& (year-m)%19== 0 {
            break;
        }
        year += 1;
    }
    println!("{}", year);
    println!("Time Elaped: {:?}", start.elapsed());
}
fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}