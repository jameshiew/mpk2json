use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{stdout, BufReader, BufWriter},
    path::PathBuf,
};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(help = "Path to MessagePack file")]
    msgpack_file: PathBuf,
}

fn main() -> Result<()> {
    let Cli { msgpack_file } = Cli::parse();
    let file = File::open(msgpack_file)?;
    let value: rmpv::Value = rmp_serde::from_read(BufReader::new(file))?;
    serde_json::to_writer_pretty(BufWriter::new(stdout()), &value)?;
    Ok(())
}
