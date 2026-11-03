use anyhow::Result;
use clap::Parser;
use telemetry::telemetry;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    production: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = telemetry::Config { 
        production_mode: cli.production, 
        ..Default::default() 
    };
    let mut component = telemetry::new(config);
    
    component.initialize()?;
    component.start()?;
    
    println!("SigmaOS-telemetry running");
    Ok(())
}
