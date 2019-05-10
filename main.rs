extern crate sled;

use sled::{self, Db};

fn main() {
    let config = sled::ConfigBuilder::default().temporary(true).build();
    let tree = Db::start(config).unwrap();
    let key: [u8; 1] = [1];
    let result = tree.get(&key);
    assert!(result == None);
    tree.set(&key, Vec::new()).unwrap();
    tree.get(&key).unwrap();
}