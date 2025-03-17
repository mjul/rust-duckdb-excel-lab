use anyhow::Result;
use calamine::{Range, Reader, Xlsx, open_workbook};
use clap::Parser;
use polars::prelude::*;
use rust_duckdb_excel_lab::xlsx_to_parquet; // Import from the library
use std::fs::File;

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
}

fn main() -> Result<()> {
    let args = Args::parse();
    xlsx_to_parquet(&args.input, &args.sheet, &args.output)?;
    Ok(())
}
