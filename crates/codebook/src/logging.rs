#[cfg(test)]
pub fn init_test_logging() {
    let _ = env_logger::builder().is_test(true).try_init();
}
