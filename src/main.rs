extern crate sled;

fn main() {
    let config = sled::ConfigBuilder::default().temporary(true).build();
    let tree = sled::Db::start(config).unwrap();
    let key: [u8; 1] = [1];
    let result = tree.get(&key).unwrap();
    assert!(result == None);
    tree.set(&key, Vec::new()).unwrap();
    tree.get(&key).unwrap().unwrap();
}
