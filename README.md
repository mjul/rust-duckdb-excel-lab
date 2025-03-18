# Rust DuckDB Excel Lab

Importing large Excel sheets into DuckDB for efficient querying.

We use Calamine for converting the Excel workbook to a Parquet file.

Then, we load it into DuckDB and use DuckDB to query it.

# DuckDB

## DuckDB client

You can install the DuckDB client (.exe) with winget:

```
winget install DuckDB.cli
```

<https://duckdb.org/docs/installation/?version=stable&environment=cli&platform=win&download_method=package_manager>

Normally, it installs somewhere in the user's local AppData, *e.g.*:

```
C:\Users\marti\AppData\Local\Microsoft\WinGet\Packages\DuckDB.cli_Microsoft.Winget.Source_8wekyb3d8bbwe
```

## DuckDB library

To compile the Rust DuckDB crate it needs to link the DuckDB library, so you have to build that.
For that, install the Visual Studio build tools like this:

```
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

- `--add Microsoft.VisualStudio.Workload.VCTools`: Adds the C++ build tools workload (includes MSVC compiler, libraries,
  etc.).
- `--includeRecommended`: Adds recommended components (e.g., latest Windows SDK).

It will ask you to reboot.

After that the `cargo build` should be able to build deps. Go talk to some Go programmers while you wait. It is a lesson
in patience.

