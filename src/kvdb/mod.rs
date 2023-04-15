pub mod memdb;
pub mod error;

use dashmap;

pub type Result<T> = std::result::Result<T, error::KVDBError>;

pub trait KVDB {
    fn hset(&self, key: String, field: String, value: String) -> Result<()>;
    fn hget(&self, key: &str, field: &str) -> Result<String>;
    fn hgetall(&self, key: &str) -> Result<dashmap::DashMap<String, String>>;
}
