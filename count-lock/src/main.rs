// 若要跨 thread 使用(用 Send), 可以改用 `Arc`
use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

fn main() {
    // 當物件共用 ref 時, rust 的生命週期判斷會很混亂
    // 不如用 RC ref count: 計算引用次數, 歸 0 時釋放
    // rust 的 count 有點像小型 GC (garbage collection)機制
    let booty = Rc::new(Treasure { dubloons: 1000 });

    let my_map = TreasureMap::new(booty);

    let your_map = my_map.clone();
    println!("{:?}", my_map);
    println!("{:?}", your_map);

    // mutex lock

    let treasure = RwLock::new(Treasure { dubloons: 1000 });
    // 利用 scope 建立 lock, 在 RwLock.write 獲取, 在 scope 結束釋放
    {
        let mut lock = treasure.write();
        lock.unwrap().dubloons = 0;
        println!("Treasure emptied!");
    }

    println!("Treasure: {:?}", treasure);
}

#[derive(Debug)]
struct Treasure {
    dubloons: u32,
}

#[derive(Clone, Debug)]
struct TreasureMap {
    treasure: Rc<Treasure>,
}

impl TreasureMap {
    fn new(treasure: Rc<Treasure>) -> Self {
        TreasureMap { treasure }
    }
}

// #[derive(Debug)]
// struct Treasure {
//     dubloons: u32,
// }

// #[derive(Clone, Debug)]
// struct TreasureMap<'a> {
//     treasure: &'a Treasure,
// }

// impl<'a> TreasureMap<'a> {
//     fn new(treasure: &'a Treasure) -> Self {
//         TreasureMap { treasure }
//     }
// }
