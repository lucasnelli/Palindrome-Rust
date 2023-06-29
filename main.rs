use std::io;

fn main() {
    println!("Digite uma palavra:");
    let mut palavra = String::new();
    io::stdin()
        .read_line(&mut palavra)
        .expect("Erro ao ler a entrada");

    let palavra = palavra.trim().to_lowercase();

    if palavra.is_empty() {
        println!("Você não digitou nenhuma palavra.");
    } else if eh_palindromo(&palavra) {
        println!("A palavra {} é um palíndromo.", palavra);
    } else {
        println!("A palavra {} não é um palíndromo.", palavra);
    }

    // Aguarda a entrada do usuário antes de encerrar o programa
    println!("Pressione Enter para sair...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn eh_palindromo(palavra: &str) -> bool {
    let mut inicio = 0;
    let mut fim = palavra.len() - 1;

    while inicio < fim {
        if palavra.chars().nth(inicio) != palavra.chars().nth(fim) {
            return false;
        }
        inicio += 1;
        fim -= 1;
    }

    true
}