use cita_trie::{PatriciaTrie, Trie};
use cita_trie::{MemoryDB};
use hasher::HasherKeccak; // https://crates.io/crates/hasher
use std::sync::Arc;
use super::bench_tree::BenchTree;

pub struct PatriciaTrieWrap {
    trie: PatriciaTrie<MemoryDB, HasherKeccak>,
    db: Arc<MemoryDB>,
}

impl PatriciaTrieWrap {
    pub fn new(db: Arc<MemoryDB>, hasher: HasherKeccak) -> Self {
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
        // self.db.flush().unwrap();
        let root = self.trie.root().unwrap();
        self.trie = PatriciaTrie::from(self.db.clone(), self.trie.hasher.clone(), &root).unwrap();
        Some(())
    }
}
