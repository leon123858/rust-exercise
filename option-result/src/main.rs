use std::fs::read_to_string;
use std::str::FromStr;
fn main() {
    // option and result
    let some = returns_some();
    println!("{:?}", some);

    let none = returns_none();
    println!("{:?}", none);

    let value = returns_ok();
    println!("{:?}", value);

    let value = returns_err();
    println!("{:?}", value);
    // 解包 for option and result
    // 若解到 error 的 result, 或 None 的 option 會報錯
    let ip_address = std::net::Ipv4Addr::from_str("127.0.0.1").unwrap();
    println!("{:?}", ip_address);
    // unwrap_or, 解包失敗提供預設
    let default_string = "Default value".to_owned();
    let unwrap_or = returns_none().unwrap_or(default_string);
    println!("returns_none().unwrap_or(...): {:?}", unwrap_or);
    // unwrap else 解包失敗用函式提供結果
    // 語法提示: TS 中的 (arg1: number) => arg1 + 2 在 rust 是 |arg1: i64| arg1 + 2
    let unwrap_or_else = returns_none().unwrap_or_else(|| {
        format!(
            "Default value from a function at time {:?}",
            std::time::Instant::now()
        )
    });
    println!(
        "returns_none().unwrap_or_else(|| {{...}}): {:?}",
        unwrap_or_else
    );
    // .unwrap_or_default() 取用預設作為失敗時的預設
    // 等價 let my_string = maybe_undefined || "";
    // 可以利用類別自定義該類別預設
    let my_string = returns_none().unwrap_or_default();
    println!("{:?}", my_string);
    // match, 依照 case 取值
    let match_value = match returns_some() {
        Some(val) => val,
        None => "My default value".to_owned(),
    };
    println!("match {{...}}: {:?}", match_value);
    // if let , 當是 some 時, 執行後面函數
    if let Some(val) = returns_some() {
        println!("if let : {:?}", val);
    }
    // 利用 ? 操作符可以解開, 遇錯回 error, 可作為短路符
    let str = render_markdown("./README.md").unwrap_or_else(|_| "read file error".to_string());
    println!("{}", str);
}

fn render_markdown(file: &str) -> Result<String, std::io::Error> {
    let source = read_to_string(file)?;
    println!("{:?}", "Should not be exec, as this file do not exist");
    Ok(source)
}

fn returns_some() -> Option<String> {
    Some("my string".to_owned())
}

fn returns_none() -> Option<String> {
    None
}

fn returns_ok() -> Result<String, MyError> {
    Ok("This turned out great!".to_owned())
}

fn returns_err() -> Result<String, MyError> {
    Err(MyError("This failed horribly.".to_owned()))
}

#[derive(Debug)]
struct MyError(String);
