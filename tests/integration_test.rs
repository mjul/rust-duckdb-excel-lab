use std::fs;
use std::path::Path;
use polars::prelude::*;
use rust_duckdb_excel_lab::xlsx_to_parquet; // Import from the library

#[test]
fn test_xlsx_to_parquet() -> anyhow::Result<()> {
    let input_xlsx = "./data/response_times.xlsx";
    let output_parquet = "./data/response_times_test.parquet";
    let sheet_name = "Data"; // Adjust if the sheet name in response_times.xlsx is different

    // Ensure the input file exists
    assert!(Path::new(input_xlsx).exists(), "Input XLSX file not found at {}", input_xlsx);

    // Run the conversion
    xlsx_to_parquet(input_xlsx, sheet_name, output_parquet)?;

    // Verify the output file was created
    assert!(Path::new(output_parquet).exists(), "Parquet file was not created at {}", output_parquet);

    // Read and verify the Parquet file contents
    let parquet_file = fs::File::open(output_parquet)?;
    let df = ParquetReader::new(parquet_file).finish()?;

    // Basic checks on the DataFrame
    assert!(!df.is_empty(), "Parquet file is empty");
    assert!(df.height() > 0, "Parquet file has no rows");
    assert!(df.width() > 0, "Parquet file has no columns");

    // Clean up
    fs::remove_file(output_parquet)?;

    Ok(())
}