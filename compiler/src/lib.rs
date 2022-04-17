pub enum Error {
    Message(String)
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn compile() -> Result<String>{
    Ok("Hello compiler".to_string())
}
