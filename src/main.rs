use crate::config::AppConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::AppConfig::load_or_create()?;
    app::run(cfg)?;
    Ok(())
}
