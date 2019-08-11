use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::time::Instant;

use hasher::HasherKeccak; // https://crates.io/crates/hasher

use cita_trie::{PatriciaTrie, Trie};
use cita_trie::{RocksDB, DB};

// 實作 BenchTrie ，才能做 benchmark
trait BenchTree {
    fn _insert(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Option<()>;
    fn _get(&self, key: &Vec<u8>) -> Option<Vec<u8>>;
    // 獲取所有 insert, get 執行之後的梅克爾根
    fn _root(&mut self) -> Option<Vec<u8>>;
    fn _flush(&mut self) -> Option<()>;
}

struct PatriciaTrieWrap {
    trie: PatriciaTrie<RocksDB, HasherKeccak>,
    db: Arc<RocksDB>,
}

impl PatriciaTrieWrap {
    fn new(db: Arc<RocksDB>, hasher: HasherKeccak) -> Self {
        PatriciaTrieWrap {
            db: db.clone(),
            trie: PatriciaTrie::new(db.clone(), Arc::new(hasher)),
        }
    }
}

impl BenchTree for PatriciaTrieWrap {
    fn _insert(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Option<()> {
        match self.trie.insert(key.to_vec(), value.to_vec()) {
            Ok(_) => Some(()),
            _ => None,
        }
    }
    fn _get(&self, key: &Vec<u8>) -> Option<Vec<u8>> {
        match self.trie.get(&key.to_vec()) {
            Ok(ret) => ret,
            _ => None,
        }
    }
    fn _root(&mut self) -> Option<Vec<u8>> {
        match self.trie.root() {
            Ok(ret) => Some(ret),
            _ => None,
        }
    }
    fn _flush(&mut self) -> Option<()> {
        self.db.flush().unwrap();
        self.trie = PatriciaTrie::new(self.db.clone(), self.trie.hasher.clone());
        Some(())
    }
}

#[derive(Debug)]
enum Instruction {
    Get(Vec<u8>),
    Insert(Vec<u8>, Vec<u8>),
    Root,
}

fn read_progeam(mut file: File) -> Vec<Instruction> {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let mut vec = Vec::new();
    for instruction in buf.split("\n") {
        let units: Vec<&str> = instruction.split(" ").collect();
        if units[0].len() == 0 {
            continue;
        }
        println!("{:?}", units);
        match units[0] {
            "get" => {
                vec.push(Instruction::Get(hex::decode(units[1]).unwrap()));
            }
            "insert" => {
                vec.push(Instruction::Insert(
                    hex::decode(units[1]).unwrap(),
                    hex::decode(units[2]).unwrap(),
                ));
            }
            "root" => {
                vec.push(Instruction::Root);
            }
            u => {
                panic!("未知的指令： {}", u);
            }
        }
    }
    vec
}

fn exectuer<Tree: BenchTree>(program: Vec<Instruction>, tree: &mut Tree) {
    let now = Instant::now();

    for instruction in program {
        match instruction {
            Instruction::Get(key) => match tree._get(&key) {
                Some(value) => println!("get {} 得到 {}", hex::encode(key), hex::encode(value)),
                None => panic!("get {} 失敗", hex::encode(key)),
            },
            Instruction::Insert(key, value) => match tree._insert(&key, &value) {
                Some(_) => println!("insert {} {}", hex::encode(key), hex::encode(value)),
                None => panic!("insert {} {} 失敗", hex::encode(key), hex::encode(value)),
            },
            Instruction::Root => match tree._root() {
                Some(root) => println!("root = {}", hex::encode(root)),
                None => panic!("root 指令失敗"),
            },
        }
    }
    let elapsed = now.elapsed();
    println!("{}.{:06} 秒", elapsed.as_secs(), elapsed.subsec_micros());
}

fn main() -> std::io::Result<()> {
    let file = File::open("../test_data/simple").expect("無法開啓測試檔案");
    let program = read_progeam(file);

    let rocks_db = Arc::new(RocksDB::new());
    let hasher = HasherKeccak::new();
    let mut trie = PatriciaTrieWrap::new(rocks_db, hasher);

    exectuer(program, &mut trie);

    Ok(())
}
