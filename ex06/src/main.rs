use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: u32,
    description: String,
    done: bool,
}

fn get_json_data() -> Option<Vec<TodoItem>> {
    let file_content = match std::fs::read_to_string("todo.json") {
        Ok(v) => v,
        Err(_) => String::new(),
    };
    let todo_list = from_str::<Vec<TodoItem>>(&file_content).ok()?;
    Some(todo_list)
}

fn get_current_id() -> u32 {
    let file_content = match std::fs::read_to_string("curr_id.txt") {
        Ok(v) => v,
        Err(_) => String::from("0"),
    };
    let id = match file_content.parse::<u32>() {
        Ok(v) => v + 1,
        Err(_) => 1,
    };
    id
}

fn main() {
    loop {
        println!("{}", "=".repeat(50));
        println!("LISTA DE TAREFAS");
        println!("{}", "=".repeat(50));

        let mut current_data: Vec<TodoItem> = match get_json_data() {
            Some(v) => v,
            None => vec![],
        };

        for i in &current_data {
            println!(
                "{}. {} - {}",
                i.id,
                if i.done { "done" } else { "TODO" },
                i.description
            );
        }

        println!("{}", "=".repeat(50));
        println!("AÇÕES (digite apenas o número)");
        print!("{}", "=".repeat(50));
        println!(
            "
            1 - Adicionar Tarefa
            2 - (Des)Marcar como concluída
            3 - Excluir tarefa
            4 - Sair"
        );

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Erro ao ler entrada");

        println!("{}", "=".repeat(50));
        match choice.trim() {
            "1" => {
                println!("Escreva qual será a nova tarefa:");
                let mut new_task = String::new();
                std::io::stdin()
                    .read_line(&mut new_task)
                    .expect("Erro ao ler entrada");
                let id: u32 = get_current_id();
                std::fs::write("curr_id.txt", id.to_string()).expect("Erro ao salvar arquivo");
                let new_task = new_task.trim().to_string();
                current_data.push(TodoItem {
                    id: id,
                    description: new_task,
                    done: false,
                });
            }
            "2" => {
                println!("Digite o ID da tarefa que deseja atualizar:");
                let mut id_inp = String::new();
                std::io::stdin()
                    .read_line(&mut id_inp)
                    .expect("Erro ao ler entrada");

                let id_num = match id_inp.trim().parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => 0,
                };

                let pos = match current_data.iter().position(|i| i.id == id_num) {
                    Some(v) => v,
                    None => {
                        println!("Tarefa não encontrada");
                        continue;
                    }
                };

                current_data[pos].done = !current_data[pos].done;
            }
            "3" => {
                println!("Digite o ID da tarefa que deseja atualizar:");
                let mut id_inp = String::new();
                std::io::stdin()
                    .read_line(&mut id_inp)
                    .expect("Erro ao ler entrada");

                let id_num = match id_inp.trim().parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => 0,
                };

                let pos = match current_data.iter().position(|i| i.id == id_num) {
                    Some(v) => v,
                    None => {
                        println!("Tarefa não encontrada");
                        continue;
                    }
                };

                current_data.remove(pos);
            }
            "4" => {
                break;
            }
            _ => {
                println!("Opção inválida!");
            }
        }
        let new_json_data: String = to_string_pretty(&current_data).expect("Falha ao salvar dados");
        std::fs::write("todo.json", new_json_data).expect("Erro ao salvar arquivo");
    }
}
