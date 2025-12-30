use std::io;

fn celsius_to_fahrenheit(s: &str) -> Result<f64, std::num::ParseFloatError> {
    let mut value = s.parse::<f64>().unwrap();
    value = (value * 1.8) + 32.0;
    Ok(value)
}

fn fahrenheit_to_celsius(s: &str) -> Result<f64, std::num::ParseFloatError> {
    let mut value = s.parse::<f64>().unwrap();
    value = (value - 32.0) / 1.8;
    Ok(value)
}

fn main() {
    loop {
        println!("{}", "=".repeat(30));
        println!("Conversor de Temperatura");
        println!("{}", "=".repeat(30));

        println!(
            "Escolha o modo (digite apenas o número):
    1 - Celsius para Fahrenheit
    2 - Fahrenheit para Celsius
    3 - Sair"
        );

        let mut mode_number = String::new();
        io::stdin()
            .read_line(&mut mode_number)
            .expect("Falha ao ler valor");
        mode_number = mode_number.trim().to_string();

        match mode_number.as_str() {
            "1" => {
                let mut input = String::new();
                println!("Digite a temperatura (Celsius) a ser convertida (Fahrenheit):");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Falha ao ler valor");
                input = input.trim().to_string();
                match celsius_to_fahrenheit(&input) {
                    Ok(v) => println!("A temperatura ({input}ºC) em fahrenheit é {}", v),
                    Err(_) => println!("Digite um número válido"),
                }
            }
            "2" => {
                let mut input = String::new();
                println!("Digite a temperatura (Fahrenheit) a ser convertida (Celsius):");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Falha ao ler valor");
                input = input.trim().to_string();

                match fahrenheit_to_celsius(&input) {
                    Ok(v) => println!("A temperatura ({input}ºF) em celsius é {}", v),
                    Err(_) => println!("Digite um número válido"),
                }
            }
            "3" => {
                break;
            }
            _ => {
                println!("Por favor digite um valor válido");
            }
        }
    }
}
