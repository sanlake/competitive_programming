fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}

fn main() {
    let (mut n,mut p): (i64,i64) = (get_int(),1);

    for i in 2..=100 { 
        let aux: i64 = get_int();
        if aux > n { n=aux; p=i; }
    }

    println!("{}\n{}", n, p);
}
