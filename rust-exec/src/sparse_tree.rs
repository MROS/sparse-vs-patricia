use crate::bench_tree::BenchTree;
use cita_trie::DB;
use hashbrown::{HashMap, HashSet};
use hasher::Hasher;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct SparseTree<D, H>
where
    D: DB,
    H: Hasher,
{
    root_hash: Vec<u8>,

    db: Arc<D>,
    hasher: Arc<H>,
    cache: HashMap<Vec<u8>, Vec<u8>>,
    dirties: HashMap<Vec<u8>, Vec<u8>>,
    level_hash: Vec<Vec<u8>>, // 長度 256 ，存放 各個層級的所有葉子都爲空時的 hash
                              // passing_keys: RefCell<HashSet<Vec<u8>>>,
                              // gen_keys: RefCell<HashSet<Vec<u8>>>
}

impl<D, H> SparseTree<D, H>
where
    D: DB,
    H: Hasher,
{
    pub fn new(db: Arc<D>, hasher: Arc<H>) -> Self {
        let mut level_hash: Vec<Vec<u8>> = Vec::new();
        level_hash.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        for i in 0..255 {
            let mut pre_image = level_hash[i].clone();
            pre_image.append(&mut level_hash[i].clone());
            let hash = hasher.digest(&pre_image);
            // println!("level {} hash: {}", i + 1, hex::encode(&hash));
            level_hash.push(hash);
        }
        SparseTree {
            root_hash: level_hash[255].clone(),
            db: db.clone(),
            hasher: hasher.clone(),
            cache: HashMap::new(),
            dirties: HashMap::new(),
            level_hash: level_hash,
        }
    }
    fn insert_batch(&mut self, data: Vec<(&Vec<u8>, &Vec<u8>)>) -> Option<()> {
        Some(())
    }
}

impl<D, H> BenchTree for SparseTree<D, H>
where
    D: DB,
    H: Hasher,
{
    fn _insert(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Option<()> {
        self.dirties.insert(key.to_vec(), value.to_vec());
        Some(())
    }
    fn _get(&self, key: &Vec<u8>) -> Option<Vec<u8>> {
        match self.dirties.get(key) {
            Some(value) => Some(value.to_vec()),
            None => match self.cache.get(key) {
                Some(value) => Some(value.to_vec()),
                None => None,
            },
        }
    }
    // 執行 _root() 之後纔會更新 root
    fn _root(&mut self) -> Option<Vec<u8>> {
        let mut keys: Vec<&Vec<u8>> = self.dirties.keys().collect();

        keys.sort_by(|a, b| {
            for i in 0..(a.len()) {
                if a[i] < b[i] {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        });

        for key in keys {
            println!("{:?}", key);
        }
        // 先計算衝突點（共同前綴）

        // 開線程向上計算 hash 值

        None
    }
    fn _flush(&mut self) -> Option<()> {
        unimplemented!();
    }
}
