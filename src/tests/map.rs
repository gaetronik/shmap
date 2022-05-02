use crate::Shmap;
use rand::{distributions::Alphanumeric, Rng};
use std::time::Duration;

pub fn rand_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[test]
#[should_panic(expected = "Option::unwrap()")]
fn test_get_unknown() {
    let shmap = Shmap::new().unwrap();
    let key = rand_string(10);
    let _: String = shmap.get(&key).unwrap().unwrap();
}

#[test]
fn simple_test() {
    let shmap = Shmap::new().unwrap();
    let key = rand_string(10);
    let value = rand_string(50);

    shmap.insert(&key, value.to_owned()).unwrap();
    let ret_value: String = shmap.get(&key).unwrap().unwrap();
    assert_eq!(ret_value, value);
    shmap.remove(&key).unwrap();
}

#[test]
fn test_set_and_get() {
    let shmap = Shmap::new().unwrap();
    let key = rand_string(10);
    let value = rand_string(50);

    shmap.insert(&key, value.to_owned()).unwrap();

    let ret_value: String = shmap.get(&key).unwrap().unwrap();
    assert_eq!(ret_value, value);

    let ret_value: String = shmap.get(&key).unwrap().unwrap();
    assert_eq!(ret_value, value);

    shmap.remove(&key).unwrap();

    let key = rand_string(10);
    let value = vec!["Test".to_string(), "Vec".to_string()];

    shmap.insert(&key, value.to_owned()).unwrap();

    let ret_value: Vec<String> = shmap.get(&key).unwrap().unwrap();
    assert_eq!(ret_value, value);

    let ret_value: Vec<String> = shmap.get(&key).unwrap().unwrap();
    assert_eq!(ret_value, value);

    shmap.remove(&key).unwrap();
}

#[test]
fn test_set_and_get_big() {
    let shmap = Shmap::new().unwrap();
    let key = rand_string(10);
    let value = rand_string(5 * 1024 * 1024);

    shmap.insert(&key, value.to_owned()).unwrap();

    let ret_value: String = shmap.get(&key).unwrap().unwrap();
    assert_eq!(ret_value, value);

    let ret_value: String = shmap.get(&key).unwrap().unwrap();
    assert_eq!(ret_value, value);

    shmap.remove(&key).unwrap();
}

#[test]
fn test_remove() {
    let shmap = Shmap::new().unwrap();
    let key = rand_string(10);
    let value = rand_string(50);

    shmap.insert(&key, value).unwrap();

    shmap.remove(&key).unwrap();
}

#[test]
fn test_remove_not_found() {
    let shmap = Shmap::new().unwrap();
    let key = rand_string(10);
    shmap.remove(&key).unwrap();
}

#[test]
#[should_panic(expected = "Option::unwrap()")]
fn test_expiration() {
    let shmap = Shmap::new().unwrap();
    let key = rand_string(10);
    let value = rand_string(50);

    shmap
        .insert_with_ttl(&key, value.to_owned(), Duration::from_millis(200))
        .unwrap();
    shmap.clean().unwrap();
    let ret_value: String = shmap.get(&key).unwrap().unwrap();
    assert_eq!(ret_value, value);

    std::thread::sleep(Duration::from_millis(300));

    let shmap = Shmap::new().unwrap();
    let _: String = shmap.get(&key).unwrap().unwrap();
}

// test concurrency between set
#[test]
fn test_set_concurrency() {
    let shmap = Shmap::new().unwrap();
    let key = rand_string(10);
    let key_clone = key.clone();

    let task = move || {
        for i in 0..1024 {
            let value = rand_string(i);
            shmap.insert(&key, value).unwrap();
        }
    };

    let t1 = std::thread::spawn(task.clone());
    let t2 = std::thread::spawn(task);

    t1.join().unwrap();
    t2.join().unwrap();

    shmap.remove(&key_clone).unwrap();
}

// test concurrency between get
#[test]
fn test_get_concurrency() {
    let shmap = Shmap::new().unwrap();
    let key = rand_string(10);
    let value = rand_string(50);
    let key_clone = key.clone();

    shmap.insert(&key, value).unwrap();

    let task = move || {
        for _ in 0..1024 {
            let _: String = shmap.get(&key).unwrap().unwrap();
        }
    };

    let t1 = std::thread::spawn(task.clone());
    let t2 = std::thread::spawn(task);

    t1.join().unwrap();
    t2.join().unwrap();

    shmap.remove(&key_clone).unwrap();
}

// test concurrency between set and get
#[test]
fn test_get_set_concurrency() {
    let shmap = Shmap::new().unwrap();
    let key = rand_string(10);
    let key_clone = key.clone();

    let task = move || {
        for i in 0..1024 {
            let value = rand_string(i);
            shmap.insert(&key, value.to_owned()).unwrap();
            let _: String = shmap.get(&key).unwrap().unwrap();
        }
    };

    let t1 = std::thread::spawn(task.clone());
    let t2 = std::thread::spawn(task);

    t1.join().unwrap();
    t2.join().unwrap();

    shmap.remove(&key_clone).unwrap();
}

// test concurrency with metadatas set/remove
#[test]
fn test_metadatas_concurrency() {
    let key = rand_string(10);

    let task = move || {
        for i in 0..1024 {
            let shmap = Shmap::new().unwrap();
            let value = rand_string(i);
            shmap.insert(&key, value.to_owned()).unwrap();
            let _: Option<String> = shmap.get(&key).unwrap();
            shmap.remove(&key).unwrap();
        }
    };

    let t1 = std::thread::spawn(task.clone());
    let t2 = std::thread::spawn(task);

    t1.join().unwrap();
    t2.join().unwrap();
}
