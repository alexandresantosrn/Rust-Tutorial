fn main() {
    println!("Qual é o seu nome?");
    
    let mut nome = String::new();
    std::io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler a linha");    
    
    println!("Olá, {}! Seja bem-vindo ao mundo do Rust!" , nome.trim());
}
