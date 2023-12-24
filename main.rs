use glob::glob;
use std::env::args;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::Write;
use std::path;
use std::process::exit;
static mut numeros_de_linhas: i32 = 0;
static mut nome_arquivo: String = String::new();
static mut eliminar: isize = 1;
static head: &str = r#"

_________
/ ======= \
/ __________\
| ___________ |
| | -       | |
| |         | |
| |_________| |________________________
\=____________/ Lanbyshell  )
/ """"""""""" \                       /
/ ::::::::::::: \                 =D-'
(_________________)
                
                   "#;

fn main() {
    let args_env: Vec<String> = args().collect();

    veryfic(args_env.len()); // Ok , continue // exit(1) sair
    unsafe {
        nome_arquivo = args_env[2].clone();
        if let Some((linhas, linha_eliminar)) =
            function_convert(args_env[3].clone(), args_env[4].clone())
        {
            numeros_de_linhas = linhas;
            eliminar = linha_eliminar;
        } else {
            eprintln!("Number please");
            exit(1);
        }

        if let Ok(valor_string) = abrir_arquivo(args_env[1].clone()) {
            unsafe {
                recortar_linhas(valor_string);
            }
        } else {
            eprintln!("Arquivo não existe");
        }
    }
}
fn veryfic(len: usize) {
    if len < 5 {
        eprintln!("code.exe patch/ nome_arquivo number");
        std::process::exit(1);
    }
}

fn abrir_arquivo(caminho: String) -> Result<String, ()> {
    if let Ok(string) = read_to_string(caminho) {
        return Ok(string);
    }
    return Err(());
}

fn recortar_linhas(string_var: String) {
    unsafe {
        let mut buffer: Vec<String> = Vec::new();
        let mut contador = 0;
        let mut contador_nucleo = 0;
        for linhas in string_var.lines() {
            if 0 != eliminar {
                eliminar -= 1;
                continue;
            }
            if contador < numeros_de_linhas {
                buffer.push(format!("{linhas}\n"));

                contador += 1;
                if contador == numeros_de_linhas {
                    contador_nucleo += 1;
                    salvar_arquivo(buffer.clone(), contador_nucleo);
                    contador = 0;

                    buffer.clear()
                }
            }
        }
        contador_nucleo += 1;
        if buffer.len() > 1 {
            salvar_arquivo(buffer.clone(), contador_nucleo);
            buffer.clear();
        }
    }
}

fn salvar_arquivo(valor: Vec<String>, contador_nucleo: isize) {
    unsafe {
        if let Ok(mut arquivo) = File::create(format!("{}_{}.txt", nome_arquivo, contador_nucleo)) {
            arquivo.write(head.as_bytes());
            arquivo.write("\r\r".as_bytes());

            for c in valor {
                arquivo.write(c.as_bytes());
            }
            arquivo.write(head.as_bytes());
        }

        println!(
            "Arquivo: {} salvo com sucesso",
            format!("{}_{}.txt", nome_arquivo, contador_nucleo)
        );
    }
}

fn function_convert(valor_1: String, valor_2: String) -> Option<(i32, isize)> {
    fn convert(v: &String) -> bool {
        if let Ok(valor) = v.parse::<usize>() {
            return true;
        } else {
            return false;
        }
    }

    if convert(&valor_1) && convert(&valor_2) {
        return Some((
            valor_1.parse::<i32>().unwrap(),
            valor_2.parse::<isize>().unwrap(),
        ));
    } else {
        return None;
    }
}
