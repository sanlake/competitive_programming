fn get_int_vec() -> Vec<i64> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    let split = buffer.split_whitespace();
    let vec: Vec<&str> = split.collect();
    vec.iter().map(|x| x.trim().parse::<i64>().unwrap()).collect()
}

fn main() {
    let v: Vec<i64> = get_int_vec();

    let (a,b): (i64, i64) = (v[0], v[1]);

    if a==0 || b==0 || b%a==0 || a%b==0 { 
        println!("Sao Multiplos");
    } else { 
        println!("Nao sao Multiplos");
    }
}
