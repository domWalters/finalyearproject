use std::{
    fmt,
    env::* };
use csv::Reader;

use crate::quarter::Quarter;
use crate::data_record::{StockID, DataRecord};

#[derive(Debug)]
pub struct Quarters {
    pub field_names: Vec<String>,
    pub quarters_vector: Vec<Quarter>,
    pub starting_year: i64,
    pub starting_quarter: i64
}

impl fmt::Display for Quarters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quarters[field_names: {:?}, quarters_vector: {:?}, starting_year: {:?}, starting_quarter: {:?}]", self.field_names, self.quarters_vector, self.starting_year, self.starting_quarter)
    }
}

impl Quarters {
    /// Generate the Quarters object from the default data directory (from this files location, the
    /// folder is ../../test-data/TrimmedUnitedData).
    pub fn new_quarters_from_default_file() -> Quarters {
        let mut pre_output: Vec<Quarter> = Vec::new();
        // Populate with every blank quarter since epoch
        let (mut year_count, mut quarter_count) = (1970, 1);
        while year_count < 2019 {
            pre_output.push(Quarter::load_blank(year_count, quarter_count));
            if quarter_count == 4 {
                year_count += 1;
                quarter_count = 1;
            } else {
                quarter_count += 1;
            }
        }
        // Path to trimmed folder
        let mut trim_unite_folder = current_dir().unwrap();
        trim_unite_folder.pop(); trim_unite_folder.push("test-data/TrimmedUnitedData");
        // Files list
        let files_iter = trim_unite_folder.read_dir().unwrap().map(|r| r.unwrap()); // NOT SORTED
        // Populate vector of readers
        let mut file_readers = Vec::new();
        for file in files_iter {
            file_readers.push((Reader::from_path(file.path()).unwrap(), file.file_name().into_string().unwrap().split('_').next().unwrap().to_string()));
        }
        // Go through every file and assemble quarters
        let mut year_index = 0;
        let mut quarter_index = 0;
        let mut columns_found = false;
        let mut field_names = Vec::new();
        for (mut reader, name) in file_readers {
            // Find the year and quarter columns (only done once, all files share this column index)
            if !columns_found {
                for (i, field) in reader.headers().unwrap().iter().enumerate() {
                    if field == "year" {
                        year_index = i;
                    } else if field == "period" {
                        quarter_index = i;
                    }
                }
                field_names = reader.headers().unwrap().iter().map(|field| field.to_string()).collect();
                columns_found = true;
            }
            for row_wrapped in reader.records() {
                if let Ok(row) = row_wrapped {
                    // Get the row year and quarter as numbers
                    let year = row.get(year_index).unwrap().parse::<i64>().unwrap();
                    let quarter = row.get(quarter_index).unwrap()[1..=1].parse::<i64>().unwrap();
                    // Create the DataRecord representation of the Record
                    let mut data_record = DataRecord {
                        record: Vec::new(),
                        stock_id: StockID {
                            name: name.clone(),
                            year: year,
                            quarter: quarter
                        }
                    };
                    for (i, field) in row.iter().enumerate() {
                        if !((i == year_index) | (i == quarter_index)) {
                            let parsed_field = field.parse::<f64>();
                            match parsed_field {
                                Ok(float_field) => data_record.push(float_field),
                                Err(_err) => data_record.push(0.0), // if the field is empty, use 0
                            }
                        }
                    }
                    // Put it into the quarter it belongs to
                    pre_output.get_mut(((year - 1970) * 4 + (quarter - 1)) as usize)
                              .unwrap()
                              .push(data_record);
                }
            }
        }
        // Issue from above: Files may still start and end at different times.
        // Solution: Assemble quarters even if they don't hold enough. Then ditch them by using length after the fact.
        println!("Finding largest quarter...");
        let largest_length = pre_output.iter().fold(0, |acc, quarter| {
            let len = quarter.len();
            if len > acc {
                println!("New largest quarter {:?} with value {}", (quarter.year, quarter.quarter), len);
                len
            } else {
                acc
            }
        });

        let output: Vec<Quarter> = pre_output.into_iter().filter(|quarter| {
            let keep = quarter.len() >= (8 * largest_length) / 10;
            if !keep {
                println!("Throwing away {:?} with length of {:?}, which is below 80% of {:?} ({:?}).", (quarter.year, quarter.quarter), quarter.len(), largest_length, (4 * largest_length) / 5);
            }
            keep
        }).collect();
        let first_quarters_year = output[0].year;
        let first_quarters_quarter = output[0].quarter;
        Quarters {
            field_names: field_names,
            quarters_vector: output,
            starting_year: first_quarters_year,
            starting_quarter: first_quarters_quarter
        }
    }
    /// Calculates the average percentage gain of the whole market over time.
    ///
    /// # Arguments
    /// * `index_of_value` - The index in a DataRecord which represents the value of the stock.
    pub fn natural_gain(&self, index_of_value: usize) -> f64 {
        // Get quarters
        let starting_quarter = self.quarters_vector.first().unwrap();
        let final_quarter = self.quarters_vector.last().unwrap();
        // For each element of the first, find it in the second.
        let mut value_multiplier = Vec::new();
        for record_in_starting in &starting_quarter.quarter_vector {
            if let Some(record_in_final) = final_quarter.find_by_stock_name(&record_in_starting) {
                value_multiplier.push(record_in_final.get(index_of_value) / record_in_starting.get(index_of_value));
            } else {
                // Stock no longer existed...
            }
        }
        value_multiplier.iter().fold(0.0, |acc, f| acc + f ) / (value_multiplier.len() as f64)
    }
    /// Gets the requested index from the quarters_vector field, as an Option.
    ///
    /// # Arguments
    /// * `index` - The index requested.
    pub fn get(&self, index: usize) -> Option<&Quarter> {
        self.quarters_vector.get(index)
    }
    /// Returns the length of the quarters_vector field.
    pub fn len(&self) -> usize {
        self.quarters_vector.len()
    }
}
