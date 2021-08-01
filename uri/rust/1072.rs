fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}

fn main() {
    let (n,mut i,mut o): (i64,i64,i64) = (get_int(),0,0); 

    for _ in 0..n { 
        let aux: i64 = get_int();
        
        if aux >= 10 && aux <= 20 { i+=1; }
        else { o+=1; }
    }

    println!("{} in\n{} out", i, o);
}
