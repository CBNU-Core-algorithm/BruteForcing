fn main() {
    let n: i32 = input().parse().unwrap();
    let m = input().parse().unwrap();

    let mut min = i32::MAX;
    let mut sum = 0;

    for i in 0..100 {
        if n <= i * i && i * i <= m {
            if min > i * i {
                min = i * i;
            }
            sum += i * i;
        }
    }

    if min == i32::MAX {
        println!("-1");
    } else {
        println!("{}", sum);
        println!("{}", min);
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
