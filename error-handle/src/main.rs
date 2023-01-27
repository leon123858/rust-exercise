use std::fs::read_to_string;

// 利用錯誤處理函式庫- anyhow => 不用多餘定義可以直接使用, 最方便
fn main() -> anyhow::Result<()> {
    let html = render_markdown()?;
    println!("{}", html);
    let html = render_markdown2()?;
    println!("{}", html);
    Ok(())
}

//  利用錯誤處理函式庫- thiserror => 可以簡單自定義
#[derive(thiserror::Error, Debug)]
enum MyError2 {
    #[error("Environment variable not found")]
    EnvironmentVariableNotFound(#[from] std::env::VarError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

fn render_markdown2() -> Result<String, MyError2> {
    // 觸發錯誤 1
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    // // 觸發錯誤 2
    // let source = read_to_string("./README.md")?;
    Ok(source)
}

// 自定義版 => 完全自定義, 但成本很高
#[derive(Debug)]
enum MyError {
    EnvironmentVariableNotFound,
    IOError(std::io::Error),
}
// 說明哪兩種錯誤
impl From<std::env::VarError> for MyError {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvironmentVariableNotFound
    }
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}
// 定義錯誤
impl std::error::Error for MyError {}
// 延伸錯誤必備屬性
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::EnvironmentVariableNotFound => write!(f, "Environment variable not found"),
            MyError::IOError(err) => write!(f, "IO Error: {}", err),
        }
    }
}

fn render_markdown() -> Result<String, MyError> {
    // 觸發錯誤 1
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    // // 觸發錯誤 2
    // let source = read_to_string("./README.md")?;
    Ok(source)
}
