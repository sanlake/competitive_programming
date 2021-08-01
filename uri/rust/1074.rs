fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}

fn main() {
    let n: i64 = get_int(); 

    for _ in 0..n { 
        let aux: i64 = get_int(); 

        if aux == 0 { println!("NULL"); }
        else {
            if aux % 2 == 0 { print!("EVEN "); }
            else { print!("ODD "); }

            if aux > 0 { println!("POSITIVE"); }
            else if aux < 0 { println!("NEGATIVE"); }
        }
    }
}
