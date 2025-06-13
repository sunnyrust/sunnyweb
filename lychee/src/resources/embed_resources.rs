use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resource/"]
struct Asset;

/// get Cargo.toml
pub fn get_app_cargo_default() -> Option<rust_embed::EmbeddedFile> {
    assert!(Asset::get("Cargo.template").is_some(), "Cargo.template should exist");
    Asset::get("Cargo.template")
}
/// get app.toml
pub fn get_app_default() -> Option<rust_embed::EmbeddedFile> {
    assert!(Asset::get("app.toml.template").is_some(), "app.toml.template should exist");
    Asset::get("app.toml.template")
}
/// get config.rs
pub fn get_config_default() -> Option<rust_embed::EmbeddedFile> {
    Asset::get("config.rs.template")
}
/// get err.rs
pub fn get_err_default() -> Option<rust_embed::EmbeddedFile> {
    Asset::get("err.rs.template")
}
/// get lib.rs
pub fn get_main_lib_default() -> Option<rust_embed::EmbeddedFile> {
    Asset::get("main_lib.rs.template")
}
/// get main.rs
pub fn get_main_default() -> Option<rust_embed::EmbeddedFile> {
    Asset::get("main.rs.template")
}

/// get router.rs
pub fn get_router_default() -> Option<rust_embed::EmbeddedFile> {
    Asset::get("router.rs.template")
}