use std::env;

// export CUSTOMER_KEY='190a5bf4b3cbb6c0991967ab1c48ab30790af876720f1835cbbf3820f4f5d949'
pub fn get_customer_key() -> Option<String> {
    env::var("CUSTOMER_KEY").ok()
}
