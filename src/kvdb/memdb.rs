use dashmap;

use super::error;
use super::KVDB;

pub struct MemDB {
    db: dashmap::DashMap<String, dashmap::DashMap<String, String>>,
}

impl KVDB for MemDB {
    fn hset(&self, key: String, field: String, value: String) -> super::Result<()> {
        self.db
            .entry(key)
            .and_modify(|inner| {
                inner.insert(field.clone(), value.clone());
            })
            .or_insert(dashmap::DashMap::from_iter([(field, value)]));
        Ok(())
    }

    fn hget(&self, key: &str, field: &str) -> super::Result<String> {
        if let Some(hm) = self.db.get(key) {
            if let Some(v) = hm.get(field) {
                Ok(v.value().clone())
            } else {
                Err(error::KVDBError::KeyNotFound)
            }
        } else {
            Err(error::KVDBError::HashMapNotFound)
        }
    }

    fn hgetall(&self, key: &str) -> super::Result<dashmap::DashMap<String, String>> {
        if let Some(hm) = self.db.get(key) {
            Ok(hm.value().clone())
        } else {
            Err(error::KVDBError::HashMapNotFound)
        }
    }
}
