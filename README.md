# Customer Segment SQL Generator
This command line executable takes an Excel file containing customer numbers, cloumn header name for customer number and their corresponding customer segment ID as input and generates an SQL file containing insert statements to assign customer numbers to Intershop customer segments. The output SQL file will be named after the customer segment ID.

## Setup
- Rust : https://www.rust-lang.org/tools/install
- VSCode (recommended): https://code.visualstudio.com

## Getting started
Build using cargo(Rust's build system and package manager)
```sh
cargo build
```

To check repository code without actually compiling
```sh
cargo check
```

to make release
```sh
cargo build --release
```

## Usage
```sh
./customer_segment <excel_file> <column_header_name> <customer_segment_id>
```

- <excel_file>: Path to the Excel file containing customer numbers and segment IDs.
- <column_header_name>: Column header for the customer number
- <customer_segment_id>: The ID of the customer segment. This will be used to name the output SQL file.

## Example
Suppose we have an Excel file named customer_data.xlsx containing customer numbers and segment IDs, and we want to generate SQL statements for the customer segment with ID CG_BB_A7_Werkzaamheden. We can use the following command:

```sh
./customer_segment customer_data.xlsx Klantnummer CG_BB_A7_Werkzaamheden
```

This will generate an SQL file named CG_BB_A7_Werkzaamheden.sql containing insert statements for assigning customer numbers to the CG_BB_A7_Werkzaamheden customer segment.
