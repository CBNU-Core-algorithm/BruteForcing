use std::cmp::min;

fn main() {
    let n: u64 = input().parse().unwrap();

    let mut nums: Vec<u64> = Vec::new();
    for i in input().split_whitespace() {
        nums.push(i.parse().unwrap());
    }

    let mut sum: u64 = 0;
    for i in nums.iter() {
        sum += i;
    }
    let mut max: u64 = 0;
    for i in nums.iter() {
        if max < *i {
            max = *i;
        }
    }
    if n == 1 {
        println!("{}", sum - max);
        return;
    }

    let mut mins: Vec<u64> = Vec::new();
    mins.push(min(nums[0], nums[5]));
    mins.push(min(nums[1], nums[4]));
    mins.push(min(nums[2], nums[3]));
    mins.sort();

    let min1 = mins[0];
    let min2 = mins[0] + mins[1];
    let min3 = mins[0] + mins[1] + mins[2];

    println!("{}", {
        (min3 * 4)
            + (min2 * ((n - 1) * 4 + (n - 2) * 4))
            + (min1 * ((n - 1) * (n - 2) * 4 + (n - 2) * (n - 2)))
    });
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
