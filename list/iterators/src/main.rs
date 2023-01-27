use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
// 各種迭代器用法
fn main() {
    filter();
    find();
    for_each();
    join();
    map();
    push_pop();
    shift_unshift();
    self_def();
}

#[derive(Serialize, Deserialize)]
struct Names {
    names: Vec<String>,
}

impl Names {
    fn search<T: AsRef<str>>(&self, regex_string: T) -> impl Iterator<Item = &String> {
        let regex = regex::Regex::new(regex_string.as_ref()).unwrap();
        self.names.iter().filter(move |name| regex.is_match(name))
    }
}

fn self_def() {
    let raw = include_str!("./names.json");
    let names: Names = serde_json::from_str(raw).unwrap();
    let mut result = names.search("er$");

    println!("First 5 names that end in 'er':");

    for i in 0..5 {
        println!("{}: {}", i + 1, result.next().unwrap());
    }
}

fn filter() {
    let numbers = [1, 2, 3, 4, 5];
    let even: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", even);
}

fn find() {
    let numbers = [1, 2, 3, 4, 5];
    let mut iter = numbers.iter();
    let first_even = iter.find(|x| *x % 2 == 0);
    println!("{:?}", first_even.unwrap());
    // let second_even = iter.find(|x| *x % 2 == 0);
    // println!("{:?}", second_even.unwrap());
}

fn for_each() {
    let numbers = [1, 2, 3];
    numbers.iter().for_each(|x| println!("{}", x));
}

fn join() {
    let names = ["Sam", "Janet", "Hunter"];
    let csv = names.join(",");
    println!("{}", csv);
}

fn map() {
    let list = vec![1, 2, 3];
    let doubled: Vec<_> = list.iter().map(|num| num * 2).collect();
    println!("{:?}", doubled)
}

fn push_pop() {
    let mut list = vec![1, 2];
    list.push(3);
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
}

fn shift_unshift() {
    let mut list = VecDeque::from([1, 2]);
    list.push_front(0);
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_front());
}
