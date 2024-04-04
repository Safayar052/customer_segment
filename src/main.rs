use std::env;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use calamine::{Reader, open_workbook, Xlsx, Data};

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <arg1> <arg2> <arg3>, excel file name, column header name, customer segment id are missing", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let column_header = &args[2];
    let customer_segment_id = &args[3];

    // Create an empty vector
    let mut customer_numbers: Vec<i64> = Vec::new();

    // Create an empty vector
    let mut insert_sqls: Vec<String> = Vec::new();

    // Open the Excel file
    let mut workbook: Xlsx<_> = open_workbook(file_path).expect("Error opening Excel file");

    //let sheet = workbook.worksheet_range_at(0).unwrap().unwrap();

    //let header_row = sheet.

    // Get the first worksheet
    if let Some(Ok(sheet)) = workbook.worksheet_range_at(0) {

        // Extract the headers from the first row
        let headers: Vec<String> = sheet
            .rows()
            .next()
            .unwrap()
            .iter()
            .map(|c| c.to_string())
            .collect();

        // Find the index of the specific column header
        let column_index = headers.iter().position(|header| header == column_header);

        if let Some(_index) = column_index {
            // Iterate over rows, skipping the header row
            for row in sheet.rows().skip(1) {
                // Get the cell value from the specific column
                for cell in row.iter() {
                    match cell {
                        Data::Float(cell) => {
                            let num: i64 = *cell as i64;
                            customer_numbers.push(num);
                        }
                        Data::Int(cell) => {
                            customer_numbers.push(*cell);
                        }
                        _ => {
                            // Handle other data types if needed
                            //println!("Not a customer number!!");
                        }
                    }
                }
            }
        } else {
            println!("Column header not found");
        }

        // Iterate over rows
        // for row in sheet.rows() {
        //     // Iterate over cells in the row
        //     for cell in row.iter() {
        //         match cell {
        //             Data::Float(cell) => {

        //                 let num: i64 = *cell as i64;
        //                 customer_numbers.push(num);
        //             }
        //             _ => {
        //                 // Handle other data types if needed
        //                 //println!("Not a customer number!!");
        //             }
        //         }
        //     }
        // }
    }

    // Print the vector
    //println!("{:?}", customer_numbers);

    // remove duplicates
    let unique_set: HashSet<_> = customer_numbers.drain(..).collect();
    let unique_customer_numbers: Vec<i64> = unique_set.into_iter().collect();

    for customer_number in unique_customer_numbers {
        let sql_insert_equal = format!("INSERT INTO usergroupuserassignment (usergroupid, usergroupdomainid, userid, domainid, oca, lastmodified) SELECT ug.id, ug.domainid, bp.uuid, NULL, 0, SYSDATE FROM basicprofile bp, usergroup ug WHERE ug.id = '{}' AND bp.domainid = ug.domainid AND bp.businesspartnerno = '{}';", customer_segment_id, customer_number);

        let sql_insert_like = format!("INSERT INTO usergroupuserassignment (usergroupid, usergroupdomainid, userid, domainid, oca, lastmodified) SELECT ug.id, ug.domainid, bp.uuid, NULL, 0, SYSDATE FROM basicprofile bp, usergroup ug WHERE ug.id = '{}' AND bp.domainid = ug.domainid AND bp.businesspartnerno like '{}_%';", customer_segment_id, customer_number);

        insert_sqls.push(sql_insert_equal);
        insert_sqls.push(sql_insert_like);
    }

    // Open or create a file for writing
    let output_file = format!("{}.sql", customer_segment_id);
    let file = File::create(output_file).expect("Unable to create file");
    let mut buf_writer = BufWriter::new(file);

    // Iterate over the vector and write each element to the file
    for insert_sql in &insert_sqls {
        let _ = writeln!(&mut buf_writer, "{}", insert_sql);
    }

    // Flush the buffer to ensure all data is written to the file
    let _ = buf_writer.flush();
}
