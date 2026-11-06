use ::telemetry::{telemetry, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = telemetry::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.start().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-telemetry operational");
    Ok(())
}
