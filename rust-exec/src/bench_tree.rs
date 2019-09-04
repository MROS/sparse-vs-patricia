// 實作 BenchTrie ，才能做 benchmark
pub trait BenchTree {
    fn _insert(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Option<()>;
    fn _get(&self, key: &Vec<u8>) -> Option<Vec<u8>>;
    // 獲取所有 insert 執行之後的梅克爾根
    fn _root(&mut self) -> Option<Vec<u8>>;
    fn _flush(&mut self) -> Option<()>;
}
