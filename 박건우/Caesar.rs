use std::fmt::Write;
fn main() {
    let n: u32 = input().parse().unwrap();
    let mut output = String::new();

    let start = std::time::Instant::now();
    for _ in 0..n {
        let mut probable: Vec<u64> = Vec::new();
        let digits = {
            let mut vec: Vec<char> = Vec::new();
            for val in input().split_terminator("").skip(1) {
                vec.push(val.chars().next().unwrap());
            }
            vec
        };
        let digits_len = digits.len();
        let plain_text = input();
        let cipher_text = input();
        // println!("Digits: {:?}", digits);
        // println!("Plain Text: {:?}", plain_text);
        // println!("Cipher Text: {:?}", cipher_text);
        // println!("Probable : {:?}", probable);
        for shift in 0..digits_len {
            let mut shifted_text = String::with_capacity(cipher_text.len());

            // println!("{} Shift", shift);
            for ch_cipher_text in cipher_text.chars() {
                for (j, &ch_digits) in digits.iter().enumerate() {
                    if ch_digits == ch_cipher_text {
                        shifted_text.push(digits[(j + shift) % digits_len]);
                        // let (left, right) = shifted_text.split_at(i + 1);
                        // println!("{} -> {}    {} {}",cipher_text.get(i..i+1).unwrap(),digits[((j + shift) % digits_len)].to_string(),left, right);
                        break;
                    }
                }
            }

            let find: bool = {
                //using KMP Algorithm
                let str = shifted_text.into_bytes();
                let pattern = plain_text.clone().into_bytes();

                let mut part = vec![0];
                for i in 1..pattern.len() {
                    let mut j = part[i - 1];
                    while j > 0 && pattern[j] != pattern[i] {
                        j = part[j - 1];
                    }
                    part.push(if pattern[j] == pattern[i] { j + 1 } else { j });
                }

                let mut one = false;
                let mut j = 0;

                for c in str {
                    while j > 0 && c != pattern[j] {
                        j = part[j - 1];
                    }
                    if c == pattern[j] {
                        j += 1;
                    }
                    if j == pattern.len() {
                        if one == false {
                            one = true;
                        } else {
                            one = false;
                            break;
                        }
                        j = part[j - 1];
                    }
                }

                one
            };
            if find {
                // println!("push {}", ((digits_len - shift)&digits_len));
                probable.push(((digits_len - shift)%digits_len) as u64);
            }
        }
        match probable.len() {
            0 => writeln!(output, "{}", "no solution").unwrap(),
            1 => writeln!(output, "unique: {}", probable.first().unwrap()).unwrap(),
            _ => {
                write!(output, "ambigous: ").unwrap();
                for v in {
                    probable.reverse();
                    probable
                } {
                    write!(output, " {}", v).unwrap();
                }
                writeln!(output, "").unwrap()
            }
        }
    }
    println!("{}", output);
    let duration = start.elapsed();
    println!("Time Elapsed: {:?}",duration);
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
