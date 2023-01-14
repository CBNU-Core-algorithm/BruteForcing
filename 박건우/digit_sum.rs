fn main() {
    let n = input().parse().unwrap();

    for i in 0..n {
        let mut sum = i;
        let mut tmp = i;
        while tmp > 0 {
            sum += tmp % 10;
            tmp /= 10;
        }
        if sum == n {
            println!("{}", i);
            return;
        }
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}