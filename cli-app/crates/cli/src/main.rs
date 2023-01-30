use log::error;
use log::info;
use my_lib::Module;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "wasm-runner",
    about = "Sample project from https://vino.dev/blog/node-to-rust-day-1-rustup/",
    global_settings(&[
      structopt::clap::AppSettings::ColoredHelp
    ]),
)]
struct CliOptions {
    /// The WebAssembly file to load.
    #[structopt(parse(from_os_str))]
    pub(crate) file_path: std::path::PathBuf,

    /// The operation to invoke in the WASM file.
    #[structopt()]
    pub(crate) operation: String,

    /// The data to pass to the operation
    #[structopt()]
    pub(crate) data: String,
}
// just try `cargo run -p cli -- 'test.wasm' hello "World"` for this app
// hello 是 .wasm 揭露的唯一方法
// "XXX" 是要給 hello 方法的參數
// 'test.wasm' 是加載模塊的地方
fn main() {
    // 指令行工具 可用 `cargo run -p cli -- --help` 測試
    // 要實際獲取值可用, `cargo run -p cli -- './test.wasm'`
    let opt = CliOptions::from_args();
    println!("{:?}", opt);
    // 初始化輸出, 會 log 到環境變數 RUST_LOG 的地方
    // 用此指令執行: `RUST_LOG=debug cargo run -p cli`
    // 可以過濾 log level ex: `RUST_LOG=cli=debug cargo run -p cli`
    env_logger::init();
    match run(opt) {
        Ok(output) => {
            println!("{output}");
            info!("Done");
        }
        Err(e) => {
            error!("Module failed to load: {}", e);
            std::process::exit(1);
        }
    };
}

fn run(options: CliOptions) -> anyhow::Result<String> {
    let module = Module::from_file(&options.file_path)?;
    info!("Module loaded");

    let bytes = rmp_serde::to_vec(&options.data)?;
    let result = module.run(&options.operation, &bytes)?;
    let unpacked: String = rmp_serde::from_read_ref(&result)?;

    Ok(unpacked)
}
