use base_converter::base_to_base;

fn main() {
    loop {
        println!("{}", "=".repeat(40));
        println!("Conversor de base");
        println!("{}", "=".repeat(40));
        println!(
            "Escolha a opção de entrada (Digite apenas o número):
            1 - Base 10 (decimal)
            2 - Base 16 (hexadecimal)
            3 - Base 8 (octal)
            4 - Base 2 (binário)
            5 - Sair"
        );
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");
        let input_base: &str = match input.trim() {
            "1" => "0123456789",
            "2" => "0123456789abcdef",
            "3" => "01234567",
            "4" => "01",
            "5" => break,
            _ => continue,
        };

        println!(
            "Escolha a opção de saída (Digite apenas o número):
            1 - Base 10 (decimal)
            2 - Base 16 (hexadecimal)
            3 - Base 8 (octal)
            4 - Base 2 (binário)
            5 - Sair"
        );
        
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        let output_base: &str = match input.trim() {
            "1" => "0123456789",
            "2" => "0123456789abcdef",
            "3" => "01234567",
            "4" => "01",
            "5" => break,
            _ => continue,
        };

        println!("Digite o valor a ser convertido:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        let res = match base_to_base(input.trim(), input_base, output_base) {
            Ok(v) => v,
            Err(_) => String::from("Valor ou base inválida"),
        };

        println!("{}", res);
    }
}
