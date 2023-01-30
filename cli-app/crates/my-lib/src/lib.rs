use log::{debug, trace};
use std::{fs, path::Path};
use wapc::WapcHost;
pub mod error;
use error::Error;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Module {
    host: wapc::WapcHost,
}

impl Module {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        println!("Loading wasm file from {:?}", path.as_ref());
        let bytes = fs::read(path.as_ref())
            .map_err(|e| Error::FileNotReadable(path.as_ref().to_path_buf(), e.to_string()))?;
        Self::new(&bytes)
    }
    pub fn new(bytes: &[u8]) -> Result<Self, error::Error> {
        let engine = wasmtime_provider::WasmtimeEngineProvider::new(bytes, None);
        let host = WapcHost::new(Box::new(engine), |_id, binding, ns, operation, payload| {
            trace!(
                "Guest called: binding={}, namespace={}, operation={}, payload={:?}",
                binding,
                ns,
                operation,
                payload
            );
            Err("Not implemented".into())
        })?;
        Ok(Self { host })
    }
    pub fn run(&self, operation: &str, payload: &[u8]) -> Result<Vec<u8>, Error> {
        debug!("Invoking {}", operation);
        let result = self.host.call(operation, payload)?;
        Ok(result)
    }
}

// 除非 test 模塊開啟, 不然跳過編譯
#[cfg(test)]
mod tests {
    use super::*;
    // 以下方法為一項單元測試
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn loads_wasm_file() {
        // 測試的目錄試所在 lib 的目錄, 所以要有另一個 test.wasm
        let result = Module::from_file("test.wasm");
        assert!(result.is_ok());
    }
    #[test]
    fn runs_operation() -> Result<(), Error> {
        let module = Module::from_file("test.wasm")?;

        let bytes = rmp_serde::to_vec("World").unwrap();
        let payload = module.run("hello", &bytes)?;
        let unpacked: String = rmp_serde::decode::from_read_ref(&payload).unwrap();
        assert_eq!(unpacked, "Hello, World.");
        Ok(())
    }
}
