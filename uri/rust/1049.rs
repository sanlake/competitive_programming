fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let (mut p, mut m): (i64,bool) = (0,false);
    let v: Vec<&str> = vec!["aguia", "pomba", "homem", "vaca", "pulga", "lagarta", "sanguessuga", "minhoca"];

    for _ in 0..3 {
        let str: String = get_input();

        if str.trim() == "invertebrado" { p+=4; }
        else if str.trim() == "anelideo" || str.trim() == "mamifero" { p+=2; }
        else if str.trim() == "onivoro" || str.trim() == "herbivoro" { p+=1; }

        if str.trim() == "mamifero" { m=true; }
        if m && str.trim() == "onivoro" { p-=1; }
    }

    println!("{}", v[p as usize]);
}
