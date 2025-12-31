use rand::Rng;

fn get_number(msg: &str) -> Result<u32, std::num::ParseIntError> {
    println!("{}", msg);
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Falha ao ler entrada");
    inp.trim().parse::<u32>().into()
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Records {
    tries: u32,
    max_number: u32,
}

fn get_json_data() -> Option<Vec<Records>> {
    let file_content = match std::fs::read_to_string("records.json") {
        Ok(v) => v,
        Err(_) => String::new(),
    };
    let records = serde_json::from_str::<Vec<Records>>(&file_content).ok()?;
    Some(records)
}

fn generate_random_number(max: u32) -> u32 {
    // let mut rng = rand::thread_rng();
    let n = rand::rng().random_range(1..=max);
    n
}

fn main() {
    println!("{}", "=".repeat(30));
    println!("Iniciando Jogo de Adivinhação");
    println!("{}", "=".repeat(30));

    let max = match get_number("Insira o valor máximo, quanto maior, mais difícil:") {
        Ok(v) => v,
        Err(_) => {
            println!("Número inválido\no número máximo foi definido como 100");
            100
        }
    };

    let r_num = generate_random_number(max);
    let mut guesses: u32 = 0;
    let mut guesses_list: Vec<u32> = vec![];
    let mut records = match get_json_data() {
        Some(v) => v,
        None => vec![]
    };

    println!("Jogo iniciado");
    let mut guess = 0;
    while guess != r_num {
        guess = match get_number("Insira sua tentativa:") {
            Ok(v) => v,
            Err(_) => {
                println!("Número inválido");
                0
            }
        };
        if guess > r_num {
            println!("Número muito ALTO ⬇\n")
        }
        if guess < r_num {
            println!("Número muito BAIXO ⬆\n")
        }
        guesses += 1;
        guesses_list.push(guess)
    }
    println!("Você adivinhou, parabéns");
    println!("{} Tentativas:", guesses);
    print!("\t");
    for g in &guesses_list {
        if *g == guesses_list[guesses_list.len() - 1] {
            print!("*{}*", g)
        } else {
            print!("{}, ", g);
        }
    }
    println!("");
    let position = records.iter().position(|x| x.max_number == max);
    
    if let Some(pos) = position {
        if guesses < records[pos].tries {
            println!("NOVO RECORD (max {})!!", max);
            records[pos].tries = guesses;
        } else {
            println!("Recorde atual (max {}) : {}", max, records[pos].tries);
        }
    }else {
        records.push(Records {
            tries: guesses,
            max_number: max,
        });
        println!("RECORD (max {}) DEFINIDO COMO {} TENTATIVAS", max, guesses);
    }
    let json_data = serde_json::to_string_pretty(&records).expect("Erro ao converter para JSON");
    std::fs::write("records.json", json_data).expect("Erro ao salvar arquivo JSON");
}
