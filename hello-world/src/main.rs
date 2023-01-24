fn greeting(greet: String) {
    println!("{}", greet);
}

// 驗證一些 rust 規則
fn verify_law() {
    let source = std::fs::read_to_string("./test.txt").unwrap();
    let mut files = std::collections::HashMap::new();
    // source 的數據轉移只能發生一次, 因為數據具有唯一所有權,
    // 所以這裡要使用 clone 來創造兩筆數據
    files.insert("README", source.clone());
    files.insert("README2", source);
    // 前面加 & 獲取 reference, 可以不用複製下引用數據
    let files_ref = &files;
    let files_ref2 = &files;

    print_borrowed_map(files_ref);
    print_borrowed_map(files_ref2);

    // 一筆數據可做一次可變引用
    let file_ref3 = &mut files;
    needs_mutable_ref(file_ref3)
}

fn needs_mutable_ref(_map: &mut std::collections::HashMap<&str, String>) {
    println!("{:?}", _map)
}

fn print_borrowed_map(map: &std::collections::HashMap<&str, String>) {
    println!("{:?}", map)
}

fn main() {
    greeting(("Hello, world!").to_string());
    let mut tmp: u32 = 2;
    greeting(tmp.to_string());
    tmp = 3;
    greeting(tmp.to_string());
    let tmp = "var can be rewrite";
    greeting(tmp.to_string());
    verify_law()
}
