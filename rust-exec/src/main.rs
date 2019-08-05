use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use hasher::{Hasher, HasherKeccak}; // https://crates.io/crates/hasher

use cita_trie::{MemoryDB, DB};
use cita_trie::{PatriciaTrie, Trie};

// 實作 BenchTrie ，才能做 benchmark
trait BenchTree {
    fn _insert(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<()>;
    fn _get(&self, key: Vec<u8>) -> Option<Vec<u8>>;
    // 獲取所有 insert, get 執行之後的梅克爾根
    fn _root(&mut self) -> Option<Vec<u8>>;
}

impl<D, H> BenchTree for PatriciaTrie<D, H>
where
    D: DB,
    H: Hasher,
{
    fn _insert(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<()> {
        match self.insert(key, value) {
            Ok(_) => Some(()),
            _ => None,
        }
    }
    fn _get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        match self.get(&key) {
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

enum Instruction {
    get(Vec<u8>),
    insert(Vec<u8>, Vec<u8>),
    root,
}

fn read_progeam(mut file: File) -> Vec<Instruction> {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let mut vec = Vec::new();
    vec.push(Instruction::root);
    vec
}

fn exectuer<Tree: BenchTree>(program: Vec<Instruction>, tree: &Tree) {}

fn main() {
    let memdb = Arc::new(MemoryDB::new(true));
    let hasher = Arc::new(HasherKeccak::new());

    let key = "test-key".as_bytes();
    let value = "test-value".as_bytes();

    let root = {
        let mut trie = PatriciaTrie::new(Arc::clone(&memdb), Arc::clone(&hasher));
        trie.insert(key.to_vec(), value.to_vec()).unwrap();

        let v = trie.get(key).unwrap();
        assert_eq!(Some(value.to_vec()), v);
        trie.root().unwrap()
    };

    let mut trie = PatriciaTrie::from(Arc::clone(&memdb), Arc::clone(&hasher), &root).unwrap();
    let exists = trie.contains(key).unwrap();
    assert_eq!(exists, true);
    let removed = trie.remove(key).unwrap();
    assert_eq!(removed, true);
    let new_root = trie.root().unwrap();
    println!("new root = {:?}", new_root);
}
