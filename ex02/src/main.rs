use std::io;

fn read_number(prompt: &str) -> Result<f64, std::num::ParseFloatError> {
    println!("{}", prompt.to_string());
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("falha");
    inp.trim().parse()

}

fn op_sum() -> Result<f64, std::num::ParseFloatError> {
    let v1: f64 = read_number("Digite o primeiro valor")?;
    let v2: f64 = read_number("Digite o segundo valor")?;
    Ok(v1 + v2)
}

fn op_sub() -> Result<f64, std::num::ParseFloatError> {
    let v1: f64 = read_number("Digite o primeiro valor")?;
    let v2: f64 = read_number("Digite o segundo valor")?;
    Ok(v1 - v2)
}

fn op_mult() -> Result<f64, std::num::ParseFloatError> {
    let v1: f64 = read_number("Digite o primeiro valor")?;
    let v2: f64 = read_number("Digite o segundo valor")?;
    Ok(v1 * v2)
}

fn op_div() -> Result<f64, std::num::ParseFloatError> {
    let v1: f64 = read_number("Digite o primeiro valor")?;
    let v2: f64 = read_number("Digite o segundo valor")?;
    if v2 == 0.0 {
        println!("Erro: Divisão por zero não é permitida.");
        return Err("".parse::<f64>().unwrap_err());
    }
    Ok(v1 / v2)
}

fn main() {
    loop {
        println!("{}","=".repeat(30));
        println!("Calculadora Simples");
        println!("{}","=".repeat(30));

        let mut choice = String::new();
        println!("Escolha o modo:
        1 - Soma (+)
        2 - Subtração (-)
        3 - Multiplicação (*)
        4 - Divisão (/)
        5 - Sair
        ");

        io::stdin()
            .read_line(&mut choice)
            .expect("Erro ao ler input");
        
        match choice.trim().to_lowercase().as_str(){
            "1" | "soma" | "+" | "(+)" => {
                match op_sum(){
                    Ok(v) => println!("O resultado da operação foi {v}"),
                    Err(_) => println!("Por favor, digite valores válidos")
                }
            }
            "2" | "subtração" | "-" | "(-)" => {
                match op_sub(){
                    Ok(v) => println!("O resultado da operação foi {v}"),
                    Err(_) => println!("Por favor, digite valores válidos")
                }
            }
            "3" | "multiplicação" | "*" | "(*)" => {
                match op_mult(){
                    Ok(v) => println!("O resultado da operação foi {v}"),
                    Err(_) => println!("Por favor, digite valores válidos")
                }
            }
            "4" | "divisão" | "/" | "(/)" => {
                match op_div(){
                    Ok(v) => println!("O resultado da operação foi {v}"),
                    Err(_) => println!("Por favor, digite valores válidos")
                }
            }
            "5" | "sair" => break,
            _ => println!("Opção inválida")
        }

    }
}
