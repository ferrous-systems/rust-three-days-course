use std::sync::Mutex;
use std::collections::VecDeque;

pub struct SyncedMailbox {
    inner: Mutex<VecDeque<String>>,
}

impl SyncedMailbox {
    pub fn new() -> SyncedMailbox {
        let inner = Mutex::new(VecDeque::new());
        SyncedMailbox { inner: inner }
    }

    pub fn write(&self, message: String) {
        let mut vector = self.inner.lock().unwrap();
        vector.push_back(message);
    }

    pub fn read(&self) -> Option<String> {
        let mut vector = self.inner.lock().unwrap();
        vector.pop_front()
    }
}
