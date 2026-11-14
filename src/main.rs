use std::process::Command;

fn system(cmd: &str) -> i32 {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .map(|s| s.code().unwrap_or(-1))
        .unwrap_or(-1)
}

fn sh(cmd: &str) {
    system(cmd);
}

#[allow(dead_code)]
fn log(mensagem: &str) {
    println!("{}", mensagem);
}

fn main() {
    sh("echo echo");
    println!("iniciado!!!")
}
