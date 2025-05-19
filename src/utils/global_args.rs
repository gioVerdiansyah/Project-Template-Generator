use std::collections::HashMap;
use std::sync::OnceLock;

static GLOBAL_ARGS: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn set_args(args: &HashMap<String, String>) {
    // Clone the args and store them globally
    let args_clone = args.clone();
    let _ = GLOBAL_ARGS.set(args_clone);
}

pub fn get_args() -> &'static HashMap<String, String> {
    GLOBAL_ARGS.get_or_init(|| HashMap::new())
}

pub fn get_arg(key: &str) -> Option<&'static String> {
    get_args().get(key)
}