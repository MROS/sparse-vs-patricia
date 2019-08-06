use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::time::Instant;

use hasher::{Hasher, HasherKeccak}; // https://crates.io/crates/hasher

use cita_trie::{MemoryDB, DB};
use cita_trie::{PatriciaTrie, Trie};

// 實作 BenchTrie ，才能做 benchmark
trait BenchTree {
    fn _insert(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Option<()>;
    fn _get(&self, key: &Vec<u8>) -> Option<Vec<u8>>;
    // 獲取所有 insert, get 執行之後的梅克爾根
    fn _root(&mut self) -> Option<Vec<u8>>;
}

impl<D, H> BenchTree for PatriciaTrie<D, H>
where
    D: DB,
    H: Hasher,
{
    fn _insert(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Option<()> {
        match self.insert(key.to_vec(), value.to_vec()) {
            Ok(_) => Some(()),
            _ => None,
        }
    }
    fn _get(&self, key: &Vec<u8>) -> Option<Vec<u8>> {
        match self.get(&key.to_vec()) {
            Ok(ret) => ret,
            _ => None,
        }
    }
    fn _root(&mut self) -> Option<Vec<u8>> {
        match self.root() {
            Ok(ret) => Some(ret),
            _ => None,
        }
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
    let file = File::open("../program")?;
    let program = read_progeam(file);

    let memdb = Arc::new(MemoryDB::new(true));
    let hasher = Arc::new(HasherKeccak::new());
    let mut trie = PatriciaTrie::new(Arc::clone(&memdb), Arc::clone(&hasher));

    exectuer(program, &mut trie);

    Ok(())
}
