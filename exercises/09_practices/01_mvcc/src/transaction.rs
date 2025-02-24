use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use crate::engine::{Engine, Key};

use lazy_static::lazy_static;

static VERSION: AtomicU64 = AtomicU64::new(1);

lazy_static! {
    static ref ACTIVE_TXN: Arc<Mutex<HashMap<u64, Vec<Vec<u8>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub struct Transaction {
    engine: Arc<Mutex<Engine>>,
    version: u64,
    active_xid: HashSet<u64>,
}

impl Transaction {
    pub fn begin(engine: Arc<Mutex<Engine>>) -> Self {
        let version = VERSION.fetch_add(1, Ordering::Relaxed);

        let mut active_txn = ACTIVE_TXN.lock().unwrap();
        let active_xid = active_txn.keys().cloned().collect();
        active_txn.insert(version, vec![]);

        Self {
            engine: engine,
            version: version,
            active_xid: active_xid,
        }
    }

    pub fn set(&self, key: &[u8], value: Vec<u8>) {
        self.write(key, Some(value))
    }

    pub fn delete(&self, key: &[u8]) {
        self.write(key, None)
    }

    fn write(&self, key: &[u8], value: Option<Vec<u8>>) {
        let mut engine = self.engine.lock().unwrap();

        for (enc_key, _) in engine.iter().rev() {
            let key_version = Key::decode(enc_key);
            if key_version.raw_key.eq(key) {
                if !self.is_visible(key_version.version) {
                    panic!("serilization error, try again");
                }
                break;
            }
        }

        let mut active_txn = ACTIVE_TXN.lock().unwrap();
        active_txn
            .entry(self.version)
            .and_modify(|keys| keys.push(key.to_vec()))
            .or_insert_with(|| vec![key.to_vec()]);

        let enc_key = Key {
            raw_key: key.to_vec(),
            version: self.version,
        };

        engine.insert(Key::encode(enc_key), value);
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let engine = self.engine.lock().unwrap();
        for (k, v) in engine.iter().rev() {
            let key_version = Key::decode(k);
            if key_version.raw_key.eq(key) {
                return v.clone();
            }
        }
        None
    }

    pub fn commit(&self) {
        let mut active_txn = ACTIVE_TXN.lock().unwrap();
        active_txn.remove(&self.version);
    }

    fn rollback(&self) {
        let mut active_txn = ACTIVE_TXN.lock().unwrap();
        if let Some(keys) = active_txn.get(&self.version) {
            let mut engine = self.engine.lock().unwrap();
            for k in keys {
                let enc_key = Key {
                    raw_key: k.to_vec(),
                    version: self.version,
                };
                let res = engine.remove(&Key::encode(enc_key));
                assert!(res.is_some())
            }
        }
        active_txn.remove(&self.version);
    }

    fn is_visible(&self, version: u64) -> bool {
        if self.active_xid.contains(&version) {
            return false;
        }
        self.version >= version
    }

    // 打印出所有可见的数据
    pub fn print_all(&self) {
        let mut records: BTreeMap<Vec<u8>, Option<Vec<u8>>> = BTreeMap::new();
        let kvengine = self.engine.lock().unwrap();
        for (k, v) in kvengine.iter() {
            let key_version = Key::decode(k);
            if self.is_visible(key_version.version) {
                records.insert(key_version.raw_key.to_vec(), v.clone());
            }
        }

        for (k, v) in records.iter() {
            if let Some(value) = v {
                print!(
                    "{}={} ",
                    String::from_utf8_lossy(k),
                    String::from_utf8_lossy(value)
                );
            }
        }
        println!("");
    }
}
