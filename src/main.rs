use anyhow::Result;
use clap::Parser;
use rust_duckdb_excel_lab::xlsx_to_parquet; // Import from the library
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Optional metadata
struct Args {
    /// Path to the input XLSX file
    #[arg(short, long)]
    input: String,

    /// Name of the sheet with the data in the Excel workbook.
    #[arg(short, long, default_value = "Data")]
    sheet: String,

    /// Path to the output Parquet file
    #[arg(short, long)]
    output: String,

    /// Path to the output DuckDB file
    #[arg(short, long)]
    database: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input_path = Path::new(args.input.as_str());
    let output_path = Path::new(args.output.as_str());
    xlsx_to_parquet(&input_path, &args.sheet, &output_path)?;
    Ok(())
}
