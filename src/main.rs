mod args;
mod scanner;
mod metadata;
mod api;
mod processor;

use clap::Parser;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();

    let base_dir = std::fs::canonicalize(&args.base_dir)
        .context(format!("Failed to process base directory: {:?}", args.base_dir))?;
    
    let input_path = std::fs::canonicalize(&args.input)
        .context(format!("Failed to process input path: {:?}", args.input))?;

    let output_dir = if args.output_dir.exists() {
        std::fs::canonicalize(&args.output_dir)?
    } else {
        std::fs::create_dir_all(&args.output_dir).context("Failed to create output directory root")?;
        std::fs::canonicalize(&args.output_dir)?
    };

    let files = scanner::find_audio_files(&input_path);
    
    if files.is_empty() {
        println!("No audio files found in {:?}", input_path);
        return Ok(());
    }

    println!("Found {} files to process.", files.len());

    let processor = processor::Processor::new(base_dir, output_dir);

    for file in files {
        if let Err(e) = processor.process_file(&file).await {
            eprintln!("Critical error processing {:?}: {}", file, e);
        }
        println!();
    }

    Ok(())
}
