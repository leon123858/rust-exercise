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

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn blocks_always_return() {
    let apples = 6;
    let message = if apples > 10 {
        "Lots of apples"
    } else if apples > 4 {
        "A few apples"
    } else {
        "Not many apples at all"
    };
    println!("{}", message) // prints "A few apples"
}

fn visible_return() -> i32 {
    let x = 10;
    if x == 10 {
        return 10;
    }
    20
}

fn hide_return() -> i64 {
    10
}

fn main() {
    greeting(("Hello, world!").to_string());
    let mut tmp: u32 = 2;
    greeting(tmp.to_string());
    tmp = 3;
    greeting(tmp.to_string());
    let tmp = "var can be rewrite";
    greeting(tmp.to_string());
    verify_law();
    // check string type
    print_type(&"test str");
    print_type(&String::new());
    let string_hi: String = ("Hi!").to_string();
    // 直接寫不是 string, 因為有一個 string slice 優化
    // 會把重複的 string slice 重用, 檔一空間, 轉成 string 才會自己成為物件
    assert!("Hi!" == string_hi);
    // 在 rust 裡什麼東西都會回傳
    println!("{:?}", blocks_always_return());
    // 隱式函數回傳
    hide_return();
    // 顯示函數回傳
    visible_return();
    // 向量(dynamic list)使用
    let mut numbers = vec![1, 2, 3, 4, 5]; // ⬅ Notice the vec! macro
    numbers.push(7);
    println!("{:?}", numbers);
    // static list
    let numbers = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    // use Hashmap just like object in js
    let mut map = std::collections::HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    // unwrap 可以解開 some, some 是一種列舉的包裝, 在 rust 中用來避免 null, undefined
    println!("{:?}", map.get("key1").unwrap_or(&""));
    println!("{:?}", map.get("key2"));
    // use struct like interface in ts
    #[derive(Debug)] // 加這行, 測試時可以無腦 print
    struct TrafficLight {
        _color: String,
    }
    let _light = TrafficLight {
        _color: "red".to_owned(), // Note we want an owned String 我们想 一个自若
    };
    // use class in rust
    impl TrafficLight {
        pub fn new() -> Self {
            println!("create a class");
            Self {
                _color: "red".to_owned(),
            }
        }
    }
    impl std::fmt::Display for TrafficLight {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Traffic light is {}", self._color)
        }
    }
    let light = TrafficLight::new();
    println!("{}", light); // 上方有實作該方法複寫
    println!("{:?}", light); // 因為有加 debug
}
