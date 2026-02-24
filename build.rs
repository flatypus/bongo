fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let _ = embed_resource::compile("icon.rc", embed_resource::NONE);
    }
}
