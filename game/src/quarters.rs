use std::{
    fmt,
    fs::*,
    env::* };
use csv::Reader;

use Quarter;
use DataRecord;
use StockID;

#[derive(Debug)]
pub struct Quarters {
    pub quarters_vector: Vec<Quarter>,
    pub starting_year: i64,
    pub starting_quarter: i64
}

impl fmt::Display for Quarters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quarters[quarters_vector: {:?}, starting_year: {:?}, starting_quarter: {:?}]", self.quarters_vector, self.starting_year, self.starting_quarter)
    }
}

impl Quarters {

    pub fn new_quarters_from_default_file() -> Quarters {
        // pre_output vector
        let mut pre_output: Vec<Quarter> = Vec::new();
        // Populate with every blank quarter since epoch
        let mut year = 1970;
        let mut quarter = 1;
        while year < 2019 {
            pre_output.push(Quarter::load_blank(year, quarter));
            if quarter == 4 {
                year += 1;
                quarter = 1;
            } else {
                quarter += 1;
            }
        }
        // Path to trimmed folder
        let mut trim_unite_folder = current_dir().unwrap();
        trim_unite_folder.pop(); trim_unite_folder.push("test-data/TrimmedUnitedData");
        // Files list
        let mut files: Vec<_> = read_dir(trim_unite_folder).unwrap().map(|r| r.unwrap()).collect(); // NOT SORTED
        let mut files_iter = files.iter();
        // Populate vector of readers
        let mut file_readers = Vec::new();
        for file in files_iter {
            file_readers.push((Reader::from_path(file.path()).unwrap(), file.file_name().into_string().unwrap().split('_').next().unwrap().to_string()));
        }
        // Go through every file and assemble quarters
        for (mut reader, name) in file_readers {
            // Find the year and quarter columns
            let mut year_index = 0;
            let mut quarter_index = 0;
            {
                let headers = reader.headers().unwrap();
                for i in 0.. headers.len() {
                    let field = headers.get(i).unwrap();
                    if field == "year" {
                        year_index = i;
                    } else if field == "period" {
                        quarter_index = i;
                    }
                }
            }
            for row_wrapped in reader.records() {
                if let Ok(row) = row_wrapped {
                    // Get the row year, quarter
                    let row_year_number = row.get(year_index).unwrap().to_string().parse::<i64>().unwrap();
                    let row_quarter_number = row.get(quarter_index).unwrap().to_string()[1..=1].parse::<i64>().unwrap();
                    // Get the quarter to put this row in
                    let mut filtered_quarters = pre_output.iter_mut().filter(|quarter| {
                        (quarter.year == row_year_number) & (quarter.quarter == row_quarter_number)
                    });
                    let mut quarter_to_use = filtered_quarters.next().unwrap();
                    // Create the DataRecord representation of the Record
                    let mut data_record = DataRecord {
                        record: Vec::new(),
                        stock_id: StockID {
                            name: name.clone(),
                            year: row_year_number,
                            quarter: row_quarter_number
                        }
                    };
                    for i in 0..row.len() {
                        if !((i == year_index) | (i == quarter_index)) {
                            let parsed_field = row.get(i).unwrap().parse::<f64>();
                            match parsed_field {
                                Ok(field) => data_record.record.push(field),
                                Err(_err) => data_record.record.push(0.0),
                            }
                        }
                    }
                    quarter_to_use.quarter_vector.push(data_record);
                }
            }
        }
        // Issue from above: Files may still start and end at different times.
        // Solution: Assemble quarters even if they don't hold enough. Then ditch them by using length after the fact.
        println!("Finding largest quarter..."); // might be able to fold or something here?
        let mut largest_length = 0;
        for quarter in &pre_output {
            let new_length = quarter.len();
            if new_length > largest_length {
                println!("New largest quarter {:?} with value {}", (quarter.year, quarter.quarter), new_length);
                largest_length = new_length;
            }
        }
        let mut output = Vec::new();    // can I utilise filter here?
        let mut first_quarter_selected = false;
        let mut starting_year = 1970;
        let mut starting_quarter = 1;
        for quarter in pre_output {
            if quarter.len() >= (4 * largest_length) / 5 {
                if !first_quarter_selected {
                    first_quarter_selected = true;
                    starting_year = quarter.year;
                    starting_quarter = quarter.quarter;
                }
                output.push(quarter);
            } else {
                println!("Throwing away {:?} with length of {:?}, which is below 80% of {:?} ({:?}).", (quarter.year, quarter.quarter), quarter.len(), largest_length, (4 * largest_length) / 5);
            }
        }
        Quarters {
            quarters_vector: output,
            starting_year: starting_year,
            starting_quarter: starting_quarter
        }
    }

    pub fn get(&self, index: usize) -> Option<&Quarter> {
        self.quarters_vector.get(index)
    }

    pub fn len(&self) -> usize {
        self.quarters_vector.len()
    }

}
