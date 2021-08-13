fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}

fn get_int_vec() -> Vec<i64> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    let split = buffer.split_whitespace();
    let vec: Vec<&str> = split.collect();
    vec.iter().map(|x| x.trim().parse::<i64>().unwrap()).collect()
}

fn main() {
    let n: i64 = get_int();

    for _ in 0..n {
        let v: Vec<i64> = get_int_vec();
        let (mut a,mut b): (i64, i64) = (v[0], v[1]);

        if a > b { a ^= b; b ^= a; a ^= b; }
        if a % 2 != 0 { a+=1; }

        let mut sum: i64 = 0;
        for i in (a+1..b).step_by(2) { sum+=i; }

        println!("{}",sum);
    }
}
