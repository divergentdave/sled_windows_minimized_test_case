extern crate sled;

fn main() {
    let config = sled::ConfigBuilder::default().temporary(true).build();
    println!("Setting up tree");
    let tree = sled::Db::start(config).unwrap();
    let key: [u8; 1] = [1];
    println!("Doing first read on empty tree");
    let result = tree.get(&key).unwrap();
    assert!(result == None);
    println!("Setting value");
    tree.set(&key, Vec::new()).unwrap();
    println!("Reading back value");
    tree.get(&key).unwrap().unwrap();
    println!("Done!");
}
