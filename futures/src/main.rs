use async_std::prelude::*;
use async_std::fs::File;
use async_std::io;

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

#[async_std::main]
async fn main() -> std::io::Result<()>{
    let hello = read_file("hello").await?;
    println!("{:?}", hello);
    Ok(())
}
