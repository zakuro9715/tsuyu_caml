pub enum Error {
    Message(String)
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn compile() -> Result<String>{
    Ok("Hello compiler".to_string())
}

pub fn run() -> Result<std::process::Output> {
    use std::process::Command;
    let compiled = compile() ?;
    Ok(
        Command::new("echo")
            .arg(compiled)
            .output()
            .expect("faild to spawn command")
    )
}
