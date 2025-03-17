use anyhow::Result;
use calamine::{Range, Reader, Xlsx, open_workbook};
use clap::Parser;
use polars::prelude::*;
use std::fs::File;

fn xlsx_to_parquet(xlsx_path: &str, parquet_path: &str) -> Result<()> {
    // Open the XLSX file
    let mut workbook: Xlsx<_> = open_workbook(xlsx_path)?;

    // Assume we're working with the first sheet
    let sheet_name = workbook.sheet_names()[0].clone();
    let range: Range<_> = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| anyhow::anyhow!("Sheet not found: {}, error: {}", sheet_name, e))?;

    // Collect headers (first row)
    let headers: Vec<String> = range
        .rows()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No data in sheet"))?
        .iter()
        .map(|cell| cell.to_string())
        .collect();

    // Collect data (remaining rows)
    let mut columns: Vec<Vec<String>> = vec![Vec::new(); headers.len()];
    for row in range.rows().skip(1) {
        for (i, cell) in row.iter().enumerate() {
            columns[i].push(cell.to_string());
        }
    }

    // Create a Polars DataFrame
    let mut df = DataFrame::default();
    for (header, column) in headers.iter().zip(columns.iter()) {
        let series = Series::new(header.into(), column);
        df = df.hstack(&[series.into()])?;
    }

    // Write DataFrame to Parquet
    let mut file = File::create(parquet_path)?;
    ParquetWriter::new(&mut file).finish(&mut df)?;

    println!("Successfully converted {} to {}", xlsx_path, parquet_path);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Optional metadata
struct Args {
    /// Path to the input XLSX file
    #[arg(short, long)]
    input: String,

    /// Path to the output Parquet file
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    xlsx_to_parquet(&args.input, &args.output)?;
    Ok(())
}
