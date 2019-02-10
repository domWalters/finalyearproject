use std::env::*;
use csv::Writer;
use rand;
use rand::Rng;

pub fn build_fake_data (num_stocks: usize, num_col: usize, num_rec: usize, relationship_indicies: Vec<usize>) {
    // Path
    let mut trim_unite_folder = current_dir().unwrap();
    trim_unite_folder.pop(); trim_unite_folder.push("test-data/TrimmedUnitedData/arbitrary_trash.csv");
    // Rng
    let mut rng = rand::thread_rng();
    // Make each stock file
    for i in 0..num_stocks {
        // New writer
        let stock_file_name = format!("STCK{}_unite_trim.csv", i);
        println!("Generating {:?}...", stock_file_name);
        trim_unite_folder.set_file_name(stock_file_name);
        let mut writer = Writer::from_path(&trim_unite_folder).unwrap();
        // Make header
        let mut header_vec = Vec::new();
        for j in 0..num_col {
            if j == 0 {
                header_vec.push("adj_price".to_string());
            } else if j == num_col - 2 {
                header_vec.push("period".to_string());
            } else if j == num_col - 1 {
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
        for j in 0..num_rec {
            let mut record_vec: Vec<String> = Vec::new();
            for k in 0..num_col {
                if k == 0 {
                    // if j >= 1 {
                    //     let changes: Vec<f64> = relationship_indicies.iter().map(|&index| (records[j-1][index].parse::<f64>().unwrap() - 50.0) / 5.0).collect();
                    //     record_vec.push((records[j-1][0].parse::<f64>().unwrap() + changes.iter().fold(0.0, |acc, ele| acc + ele)).to_string());
                    if j == num_rec - 1 {
                        let mut total_change = 0.0;
                        for j in 1..num_rec {
                            total_change += relationship_indicies.iter().map(|&index| (records[j-1][index].parse::<f64>().unwrap() - 50.0) / 5.0).fold(0.0, |acc, ele| acc + ele);
                        }
                        println!("price: {:?}, change: {:?}", records[j-1][0], total_change);
                        record_vec.push((records[j-1][0].parse::<f64>().unwrap() + total_change).to_string());
                    } else {
                        record_vec.push(rng.gen_range(0.0, 100.0).to_string());
                    }
                } else if relationship_indicies.contains(&k) {
                    record_vec.push(rng.gen_range(0.0, 100.0).to_string());
                } else if k == num_col - 2 {
                    record_vec.push(format!("Q{}", quarter));
                } else if k == num_col - 1 {
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
