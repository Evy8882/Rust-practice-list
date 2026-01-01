fn main() {
    println!("{}", "=".repeat(40));
    println!("Estatísticas de Números");
    println!("{}", "=".repeat(40));

    let mut list: Vec<f64> = vec![];
    loop {
        println!("Digite um número para adicionar (inputs vazios para e exibe as estatísticas)");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler valor");
        if input.trim() == "" {
            break;
        }
        match input.trim().parse::<f64>() {
            Ok(v) => {
                list.push(v);
                println!("Valor adicionado com sucesso");
            }
            Err(_) => {
                println!("Insira apenas valores válidos")
            }
        } 
    }
    println!("");
    let size = list.len() as f64;
    if size > 0.0 {
        let mut sum: f64 = 0.0;
        let mut max = list[0];
        let mut min = list[0];
        for n in list {
            print!("{}, ", n);
            sum += n;
            if n > max {
                max = n
            }
            if n < min {
                min = n
            }
        }
        let avg = sum / size;

        println!("
        Total de números: {size},
        Soma: {sum},
        Média: {avg},
        Menor: {min},
        Maior: {max}
        ");
    }else{
        println!("Nenhum valor foi fornecido");
    }

}
