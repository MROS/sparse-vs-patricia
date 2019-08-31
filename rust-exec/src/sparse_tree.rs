use cita_trie::DB;
use std::sync::Arc;
use std::cell::RefCell;
use hashbrown::{HashMap, HashSet};
use hasher::Hasher;

pub struct SparseTree<D, H>
where
    D: DB,
    H: Hasher,
{
    root_hash: Vec<u8>,

    db: Arc<D>,
    pub hasher: Arc<H>,

    cache: RefCell<HashMap<Vec<u8>, Vec<u8>>>,
    passing_keys: RefCell<HashSet<Vec<u8>>>,
    gen_keys: RefCell<HashSet<Vec<u8>>>,
}