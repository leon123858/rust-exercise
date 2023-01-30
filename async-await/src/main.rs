#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("{}", regular_fn());
    println!("{}", async_fn(1).await);
    // rust 和 JS 不同, 執行 await 時才會開始跑
    let tmp_fn = async_fn(2);
    println!("{}", async_fn(3).await);
    println!("{}", tmp_fn.await);
    // 利用此語法可以強制包成 async
    let async_block = || async {
        println!("this is sync operation original");
    };
    async_block().await;
    // multi-process multi-thread (此概念開始無法與 JS 映射)
    let mark_twain = "Samuel Clemens".to_owned();
    async_print(mark_twain).await;
}

fn async_print<T: std::fmt::Display + Send + 'static>(msg: T) -> tokio::task::JoinHandle<()> {
    // 把 static 的 variable 傳到另一個平行的 thread print
    tokio::task::spawn(async move {
        println!("{msg}");
    })
}

fn regular_fn() -> String {
    "I'm a regular function".to_owned()
}

async fn async_fn(i: i32) -> String {
    // 实际上是 impl Future<Output = String>
    let str = format!("I'm an async function: {i}");
    str
}
