use std::env::*;
use csv::Writer;
use rand;
use rand::Rng;

pub fn build_fake_data(number_of_stocks: usize, number_of_columns: usize, number_of_records: usize, relationship_index: usize) {
    // Path
    let mut trim_unite_folder = current_dir().unwrap();
    trim_unite_folder.pop(); trim_unite_folder.push("test-data/TrimmedUnitedData/arbitrary_trash.csv");
    // Rng
    let mut rng = rand::thread_rng();
    // Make each stock file
    for i in 0..number_of_stocks {
        // New writer
        let stock_file_name = format!("STCK{}_unite_trim.csv", i);
        println!("Generating {:?}...", stock_file_name);
        trim_unite_folder.set_file_name(stock_file_name);
        let mut writer = Writer::from_path(&trim_unite_folder).unwrap();
        // Make header
        let mut header_vec = Vec::new();
        for j in 0..number_of_columns {
            if j == 0 {
                header_vec.push("adj_price".to_string());
            } else if j == number_of_columns - 2 {
                header_vec.push("period".to_string());
            } else if j == number_of_columns - 1 {
                header_vec.push("year".to_string());
            } else {
                header_vec.push(format!("mischeaderfield{}", j - 1));
            }
        }
        if let Err(_) = writer.write_record(header_vec) {
            println!("WRITE ERROR.");
        } else {
            if let Err(_) = writer.flush() {
                println!("FLUSH ERROR.");
            }
        }
        // Make rows
        let (mut year, mut quarter) = (2018, 3);
        let mut records: Vec<Vec<String>> = Vec::new();
        for j in 0..number_of_records {
            let mut record_vec: Vec<String> = Vec::new();
            for k in 0..number_of_columns {
                if k == 0 {
                    let four_yr_old_rel_value;
                    if j >= 4 {
                        four_yr_old_rel_value = records[j-4][relationship_index].parse::<f64>().unwrap();
                    } else {
                        four_yr_old_rel_value = rng.gen_range(0.0, 100.0);
                    }
                    record_vec.push(if four_yr_old_rel_value > 50.0 {rng.gen_range(50.0, 100.0).to_string()} else {rng.gen_range(0.0, 75.0).to_string()});
                } else if k == relationship_index {
                    record_vec.push(rng.gen_range(0.0, 100.0).to_string());
                } else if k == number_of_columns - 2 {
                    record_vec.push(format!("Q{}", quarter));
                } else if k == number_of_columns - 1 {
                    record_vec.push(year.to_string());
                } else {
                    record_vec.push(rng.gen_range(0.0, 100.0).to_string());
                }
            }
            records.push(record_vec.clone());
            if let Err(_) = writer.write_record(record_vec) {
                println!("WRITE ERROR.");
            } else {
                if let Err(_) = writer.flush() {
                    println!("FLUSH ERROR.");
                }
            }
            if quarter != 1 {
                quarter -= 1;
            } else {
                year -= 1;
                quarter = 4;
            }
        }
    }
}
