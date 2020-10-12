use ::lazy_static::lazy_static;
use std::{env, path::PathBuf};
lazy_static! {
    pub static ref EXAMPLES_DIR: String = format!(
        "{}/../../../tests/fixtures/Examples",
        PathBuf::from(env::current_exe().unwrap())
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
    );
}
