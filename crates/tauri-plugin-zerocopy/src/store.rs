use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct BufferEntry {
    pub data: Bytes,
    pub mime_type: String,
    pub one_time: bool,
}

#[derive(Clone, Default)]
pub struct BufferStore {
    entries: Arc<RwLock<HashMap<String, BufferEntry>>>,
}

impl BufferStore {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, id: String, data: Bytes, mime_type: String, one_time: bool) {
        let mut map = self.entries.write().await;
        map.insert(
            id,
            BufferEntry {
                data,
                mime_type,
                one_time,
            },
        );
    }

    pub async fn get(&self, id: &str) -> Option<BufferEntry> {
        let mut map = self.entries.write().await;
        if let Some(entry) = map.get(id) {
            if entry.one_time {
                map.remove(id)
            } else {
                Some(entry.clone())
            }
        } else {
            None
        }
    }

    pub async fn remove(&self, id: &str) -> bool {
        let mut map = self.entries.write().await;
        map.remove(id).is_some()
    }

    pub async fn clear(&self) {
        let mut map = self.entries.write().await;
        map.clear();
    }
}
