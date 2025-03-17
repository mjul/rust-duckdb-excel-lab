use anyhow::Result;
use calamine::{Range, Reader, Xlsx, open_workbook};
use clap::Parser;
use polars::prelude::*;
use std::fs::File;

pub fn xlsx_to_parquet(xlsx_path: &str, sheet_name: &str, parquet_path: &str) -> Result<()> {
    // Open the XLSX file
    println!("Reading spreadsheet {}...", xlsx_path);
    let mut workbook: Xlsx<_> = open_workbook(xlsx_path)?;

    let range: Range<_> = workbook.worksheet_range(&sheet_name).map_err(|e| {
        anyhow::anyhow!(
            "Sheet not found: {}. Workbook has sheets {}. Error: {}",
            sheet_name,
            workbook.sheet_names().join(", "),
            e
        )
    })?;

    // Collect headers (first row)
    let headers: Vec<String> = range
        .rows()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No data in sheet"))?
        .iter()
        .map(|cell| cell.to_string())
        .collect();

    println!("Reading {} rows from spreadsheet...", range.rows().count());

    // Collect data (remaining rows)
    let mut columns: Vec<Vec<String>> = vec![Vec::new(); headers.len()];
    for row in range.rows().skip(1) {
        for (i, cell) in row.iter().enumerate() {
            columns[i].push(cell.to_string());
        }
    }

    println!("Writing Parquet file...");

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
