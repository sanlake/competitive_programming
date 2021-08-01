fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}


fn main() {
    let mut q: i64 = 0;
    
    for _ in 0..5 { 
        let aux = get_int(); 
        if aux % 2 == 0 { q+=1; }
    }

    println!("{} valores pares", q);
}
