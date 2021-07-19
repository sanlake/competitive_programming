use std::collections::HashMap;

fn get_int() -> i32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i32>().unwrap()
}

fn main() {
    let ddd: i32 = get_int();

    let dc: HashMap<i32,&str> = vec![
        (61, "Brasilia"),
        (71, "Salvador"),
        (11, "Sao Paulo"),
        (21, "Rio de Janeiro"),
        (32, "Juiz de Fora"),
        (19, "Campinas"),
        (27, "Vitoria"),
        (31, "Belo Horizonte")
    ].iter().cloned().collect();

    if dc.contains_key(&ddd) { println!("{}", dc[&ddd]); }
    else { println!("DDD nao cadastrado"); }
}
