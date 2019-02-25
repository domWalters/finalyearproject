use std::{fmt, env::*, slice::Iter};
use csv::Reader;
use rand::Rng;

use crate::data_trait::DataTrait;
use crate::quarter::Quarter;
use crate::data_record::{TimeID, StockID, DataRecord};

#[derive(Debug)]
#[derive(Clone)]
pub struct Quarters<T: DataTrait> {
    pub field_names: Vec<String>,
    pub quarters_vector: Vec<Quarter<T>>,
    pub starting_time: TimeID,
    pub ending_time: TimeID
}

impl<T: DataTrait> fmt::Display for Quarters<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quarters[field_names: {:?}, quarters_vector: {:?}, starting_time: {}, ending_time: {}]", self.field_names, self.quarters_vector, self.starting_time, self.ending_time)
    }
}

impl<T: DataTrait> Quarters<T> {
    /// Generate the Quarters object from the default data directory (from this files location, the
    /// folder is ../../test-data/TrimmedUnitedData).
    pub fn new_quarters_from_default_file(iteration_max: usize) -> Quarters<f64> {
        let mut pre_output: Vec<Quarter<f64>> = Vec::new();
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
        let mut rng = rand::thread_rng();
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
                field_names = reader.headers().unwrap().iter().filter_map(|field| {
                    if (field != "year") && (field != "period") {
                        Some(field.to_string())
                    } else {
                        None
                    }                    
                }).collect();
                columns_found = true;
            }
            // Generate which iteration this should be used on
            let iteration = rng.gen_range(0, iteration_max);
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
                            time_id: TimeID {
                                year: year,
                                quarter: quarter
                            },
                            iteration: iteration
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
        let mut largest_time_id = TimeID {
            year: 1970,
            quarter: 1
        };
        let _largest_length = pre_output.iter().fold(0, |acc, quarter| {
            let len = quarter.len();
            if len >= acc {
                println!("New largest quarter {} with value {}", quarter.time_id.to_string(), len);
                largest_time_id = quarter.time_id.clone();
                len
            } else {
                acc
            }
        });
        let mut output: Vec<Quarter<f64>> = pre_output.into_iter().filter(|quarter| {
            let keep = largest_time_id.after(&quarter.time_id) & (quarter.quarter_vector.len() > 0);
            if !keep {
                println!("Throwing away {}.", quarter.time_id.to_string());
            }
            keep
        }).collect();
        // Now ditch all stocks that don't exist in the final quarter
        let final_quarter = &output[output.len() - 1];
        let mut indicies_to_bin = vec![Vec::new(); output.len()];
        for (i, quarter) in output.iter().enumerate() {
            'a: for (j, stock) in quarter.iter().enumerate() {
                for final_stock in final_quarter.iter() {
                    if stock.is_name(&final_stock) {
                        continue 'a;
                    }
                }
                // If you are here, the stock wasn't found.
                indicies_to_bin[i].push(j);
            }
        }
        for (i, quarter) in output.iter_mut().enumerate() {
            for j in indicies_to_bin[i].iter().rev() {
                quarter.remove(*j);
            }
        }
        let starting_time = output[0].time_id.clone();
        Quarters {
            field_names: field_names,
            quarters_vector: output,
            starting_time: starting_time,
            ending_time: largest_time_id
        }
    }
    /// Creates an ordered vector of vectors of each field of the training data.
    pub fn expensive_training_data_analysis(&self) -> Vec<Vec<T>> {
        let mut field_accumulator: Vec<Vec<T>> = vec![Vec::new(); self.get(0).unwrap().get(0).unwrap().len()];    // Vector of all results for all fields
        for current_quarter in &self.quarters_vector {
            for ref row in &current_quarter.quarter_vector {
                for (&field, field_store) in row.iter().zip(field_accumulator.iter_mut()) {
                    field_store.push(field);
                }
            }
        }
        for field_store in &mut field_accumulator {
            field_store.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        field_accumulator
    }
    ///
    ///
    /// # Arguments
    /// * `denomination` - The distance between adjacent percentiles. This number must evenly
    /// divide 100 with no remainder.
    ///
    /// # Remarks
    /// The 0th element is the lower limit of the 1st percentile. The ith element is the lower
    /// limit of the (i+1)th percentile.
    fn create_percentile_vectors(denomination: usize, training_data: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        // Compute percentile start values
        let mut percentile_boundary_vectors = vec![Vec::new(); training_data.len()];
        for (i, percentile_vector) in percentile_boundary_vectors.iter_mut().enumerate() {
            let ith_training_data = &training_data[i];
            let gap = (ith_training_data.len() as f64) / ((100 / denomination) as f64);
            for j in 0..(100 / denomination) {
                percentile_vector.push(ith_training_data[(gap as usize) + ((gap * (j as f64)) as usize)]);
            }
        }
        percentile_boundary_vectors
    }
    ///
    ///
    /// # Arguments
    /// * `denomination` - The distance between adjacent percentiles. This number must evenly
    /// divide 100 with no remainder.
    ///
    pub fn create_percentile_quarters(&self, denomination: usize, training_data: Vec<Vec<f64>>) -> Quarters<usize> {
        let percentile_boundary_vectors = Quarters::<f64>::create_percentile_vectors(denomination, training_data);
        //println!("{:?}", percentile_boundary_vectors);
        // Create new Quarters set
        let mut new_quarters_vector = Vec::new();
        for quarter in &self.quarters_vector {
            let mut new_quarter_vector = Vec::new();
            for data_record in &quarter.quarter_vector {
                let mut new_record_vector = Vec::new();
                'a: for (i, entry) in data_record.record.iter().enumerate() {
                    let percentile_vector = &percentile_boundary_vectors[i];
                    'b: for (j, element) in percentile_vector.iter().enumerate() {
                        if entry.to_f64().unwrap() > *element {
                            continue 'b;
                        } else {
                            new_record_vector.push((j + 1) * denomination);
                            continue 'a;
                        }
                    }
                }
                new_quarter_vector.push(DataRecord {
                    record: new_record_vector,
                    stock_id: data_record.stock_id.clone()
                });
            }
            new_quarters_vector.push(Quarter {
                quarter_vector: new_quarter_vector,
                time_id: quarter.time_id.clone()
            });
        }
        Quarters {
            field_names: self.field_names.clone(),
            quarters_vector: new_quarters_vector,
            starting_time: self.starting_time.clone(),
            ending_time: self.ending_time.clone()
        }
    }
    /// Gets the requested index from the quarters_vector field, as an Option.
    ///
    /// # Arguments
    /// * `index` - The index requested.
    pub fn get(&self, index: usize) -> Option<&Quarter<T>> {
        self.quarters_vector.get(index)
    }
    /// Returns the length of the quarters_vector field.
    pub fn len(&self) -> usize {
        self.quarters_vector.len()
    }
    /// Returns an iterator over references to the elements in the quarters_vector variable of
    /// the Quarters object.
    pub fn iter(&self) -> Iter<Quarter<T>> {
        self.quarters_vector.iter()
    }
}
