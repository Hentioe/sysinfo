use lazy_static::lazy_static;

lazy_static! {
    pub static ref SPROUT_AGENT_EXTERNAL_ROOT: String =
        std::env::var("SPROUT_AGENT_EXTERNAL_ROOT").unwrap_or_else(|_| String::new());
}

pub fn root_join(path: &str) -> String {
    format!("{}{}", *SPROUT_AGENT_EXTERNAL_ROOT, path)
}
