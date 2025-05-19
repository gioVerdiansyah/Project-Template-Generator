use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use std::time::Duration;
use std::thread;

pub static GLOBAL_ARGS: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub static ARGS_INITIALIZED: Lazy<Mutex<bool>> = Lazy::new(|| {
    Mutex::new(false)
});

pub fn set_args(args: &HashMap<String, String>) {
    println!("Setting global args: {:?}", args);

    match GLOBAL_ARGS.write() {
        Ok(mut global) => {
            global.clear();
            global.extend(args.clone());

            if let Ok(mut initialized) = ARGS_INITIALIZED.lock() {
                *initialized = true;
            }

            println!("Global args successfully set");
        },
        Err(e) => {
            eprintln!("Failed to acquire write lock on GLOBAL_ARGS: {:?}", e);
        }
    }
}

pub fn get_args() -> HashMap<String, String> {
    let mut attempts = 0;
    while attempts < 5 {
        if let Ok(initialized) = ARGS_INITIALIZED.lock() {
            if *initialized {
                break;
            }
        }
        thread::sleep(Duration::from_millis(10));
        attempts += 1;
    }

    match GLOBAL_ARGS.read() {
        Ok(map) => {
            let result = map.clone();
            result
        },
        Err(e) => {
            eprintln!("Failed to acquire read lock on GLOBAL_ARGS: {:?}", e);
            HashMap::new()
        }
    }
}

pub fn get_arg(key: &str) -> Option<String> {
    let mut attempts = 0;
    while attempts < 5 {
        if let Ok(initialized) = ARGS_INITIALIZED.lock() {
            if *initialized {
                break;
            }
        }
        thread::sleep(Duration::from_millis(10));
        attempts += 1;
    }

    match GLOBAL_ARGS.read() {
        Ok(map) => {
            let result = map.get(key).cloned();
            result
        },
        Err(e) => {
            eprintln!("Failed to acquire read lock on GLOBAL_ARGS for key '{}': {:?}", key, e);
            None
        }
    }
}