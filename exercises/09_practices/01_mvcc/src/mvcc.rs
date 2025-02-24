use std::sync::{Arc, Mutex};

use crate::engine::Engine;
use crate::transaction::Transaction;

pub struct MVCC {
    engine: Arc<Mutex<Engine>>,
}

impl MVCC {
    pub fn new(engine: Engine) -> Self {
        MVCC {
            engine: Arc::new(Mutex::new(engine)),
        }
    }

    pub fn begin_transaction(&self) -> Transaction {
        Transaction::begin(self.engine.clone())
    }
}
