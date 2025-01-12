use shell::Shell;
mod history;
mod parser;
mod shell;

fn main() {
    let shell = Shell::new();
    match shell {
        Ok(mut app) => app.init(),
        Err(e) => println!("Cannot init {:?}", e),
    }
}
