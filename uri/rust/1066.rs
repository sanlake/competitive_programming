fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}


fn main() {
    let (mut o,mut e,mut p,mut n):(i64,i64,i64,i64) = (0,0,0,0);
    
    for _ in 0..5 { 
        let aux = get_int(); 

        if aux % 2 == 0 { e+=1; }
        else { o+=1; }

        if aux > 0 { p+=1; }
        else if aux < 0 { n+=1; }
    }

    println!("{} valor(es) par(es)", e);
    println!("{} valor(es) impar(es)", o);
    println!("{} valor(es) positivo(s)", p);
    println!("{} valor(es) negativo(s)", n);
}
