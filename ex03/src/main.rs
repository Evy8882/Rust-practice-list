fn get_number(msg: &str) -> Result<u32, std::num::ParseIntError> {
    println!("{}", msg.to_string());
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    inp.trim().parse::<u32>().into()
}

fn main() {
    println!("{}", "=".repeat(30));
    println!("Tabuada");
    println!("{}", "=".repeat(30));

    match get_number("Digite o número que deseja ver a tabuada:") {
        Ok(n1) => match get_number("Insira o até que número deve ser multiplicado") {
            Ok(n2) => {
                for n in 1..=n2 {
                    println!("{} * {} = {}", n1, n, n * n1)
                }
            }
            Err(_) => println!("Por favor, insira apenas valores válidos"),
        },
        Err(_) => println!("Por favor, insira apenas valores válidos"),
    }
}
