mod bench_tree;
mod patricia_wrap;
mod sparse_tree;

use hasher::HasherKeccak;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::time::Instant;

use bench_tree::BenchTree;
use cita_trie::MemoryDB;
use patricia_wrap::PatriciaTrieWrap;
use sparse_tree::SparseTree;

#[derive(Debug)]
enum Instruction {
    Get(Vec<u8>),
    Insert(Vec<u8>, Vec<u8>),
    Root,
    Flush,
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
            "flush" => {
                vec.push(Instruction::Flush);
            }
            u => {
                panic!("未知的指令： {}", u);
            }
        }
    }
    vec
}

fn execute<Tree: BenchTree>(program: &Vec<Instruction>, tree: &mut Tree) {
    let now = Instant::now();

    for instruction in program {
        match instruction {
            Instruction::Get(key) => match tree._get(&key) {
                Some(_value) => {
                    // println!("get {} 得到 {}", hex::encode(key), hex::encode(value))
                }
                None => panic!("get {} 失敗", hex::encode(key)),
            },
            Instruction::Insert(key, value) => match tree._insert(&key, &value) {
                Some(_) => {
                    // println!("insert {} {}", hex::encode(key), hex::encode(value))
                }
                None => panic!("insert {} {} 失敗", hex::encode(key), hex::encode(value)),
            },
            Instruction::Root => match tree._root() {
                Some(root) => println!("root = {}", hex::encode(root)),
                None => panic!("root 指令失敗"),
            },
            Instruction::Flush => match tree._flush() {
                Some(_) => println!("flush"),
                None => panic!("flush 指令失敗"),
            },
        }
    }
    let elapsed = now.elapsed();
    println!("{}.{:06} 秒", elapsed.as_secs(), elapsed.subsec_micros());
}

fn main() -> std::io::Result<()> {
    let dir = match fs::read_dir("../test_data") {
        Err(err) => {
            println!("開啓 ../test_data 失敗: {:?}", err);
            return Err(err);
        }
        Ok(l) => l,
    };
    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            println!("{} 是一個目錄", path.to_str().unwrap());
        } else {
            let file = File::open(&path).expect("無法開啓測試檔案");
            println!("執行測試 {:?}", path);
            let program = read_progeam(file);

            let memory_db = Arc::new(MemoryDB::new(false));
            let hasher = HasherKeccak::new();
            let mut trie = SparseTree::new(memory_db, Arc::new(hasher));
            execute(&program, &mut trie);

            // let rocks_db = Arc::new(RocksDB::new());

            // let memory_db = Arc::new(MemoryDB::new(false));
            // let hasher = HasherKeccak::new();
            // let mut trie = PatriciaTrieWrap::new(memory_db, hasher);
            // exectuer(&program, &mut trie);
        }
    }

    Ok(())
}
