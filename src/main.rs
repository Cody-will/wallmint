fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = wallmint::config::AppConfig::load_or_create()?;
    wallmint::app::run(cfg);
    Ok(())
}
