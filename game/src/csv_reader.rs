pub mod csv_reader {

    use std::{
        iter::FromIterator,
        env::* };
    use csv::{Writer, Reader, StringRecord};

    fn drop_and_extend(new_record: &mut StringRecord, old_record: &StringRecord, drop_count: usize) {
        let mut iterator = old_record.iter();
        for _i in 0..drop_count {
            iterator.next();
        }
        new_record.extend(iterator);
    }

    pub fn unite_stock_csvs(stock_string: String) {
        let stock_name = stock_string.clone();

        // Configure Path
        let mut path = current_dir().unwrap();
        path.pop(); path.push("test-data/Data");

        // Open all 4 file readers
        let mut bal = stock_name.clone();
        bal.push_str(&"_fudamentals_balance.csv".to_string());
        path.push(bal);
        let mut balance = Reader::from_path(&path).unwrap();
        path.pop();

        let mut calcs = stock_name.clone();
        calcs.push_str(&"_fudamentals_calculations.csv".to_string());
        path.push(calcs);
        let mut calculations = Reader::from_path(&path).unwrap();
        path.pop();

        let mut case = stock_name.clone();
        case.push_str(&"_fudamentals_caseflow.csv".to_string());
        path.push(case);
        let mut caseflow = Reader::from_path(&path).unwrap();
        path.pop();

        let mut pri = stock_name.clone();
        pri.push_str(&"_price.csv".to_string());
        path.push(pri);
        let mut price = Reader::from_path(&path).unwrap();
        path.pop();

        // Open writer
        let mut uni = stock_name.clone();
        uni.push_str(&"_unite.csv".to_string());
        path.pop(); path.push("UnitedData"); path.push(uni);
        let mut unite = Writer::from_path(&path).unwrap();
        path.pop();

        // Construct new header
        {
            let mut case_headers_iter = caseflow.headers().unwrap().iter();
            let mut price_headers_iter = price.headers().unwrap().iter();
            price_headers_iter.next();
            let mut calcs_headers_iter = calculations.headers().unwrap().iter();
            calcs_headers_iter.next();
            calcs_headers_iter.next();
            let mut bal_headers_iter = balance.headers().unwrap().iter();
            bal_headers_iter.next();

            let mut new_headers = StringRecord::from_iter(case_headers_iter);
            new_headers.extend(price_headers_iter);
            new_headers.extend(calcs_headers_iter);
            new_headers.extend(bal_headers_iter);

            if let Err(_) = unite.write_record(&new_headers) {
                println!("WRITE ERROR WITH HEADERS.");
                std::process::exit(1);
            } else {
                if let Err(_) = unite.flush() {
                    println!("FLUSH ERROR.");
                }
            }
        }

        let mut case_records_iter = caseflow.records().peekable();
        let mut price_records_iter = price.records();
        let mut calcs_records_iter = calculations.records();
        let mut bal_records_iter = balance.records();

        let mut next_case_record = case_records_iter.next();
        let mut next_price_record = price_records_iter.next();
        let mut next_calcs_record = calcs_records_iter.next();
        let mut next_bal_record = bal_records_iter.next();

        let mut first_run = true;
        let mut written_once = false;
        let mut last_year = 0;
        let mut last_quarter = 0;
        let mut mismatch_tracker = vec![(false, false), (false, false), (false, false)];
        // {(price mismatch?, price > case?),(calcs mismatch?, calcs > case?),(bal mismatch?, bal > case?)}

        // Iterate through rows in caseflow
        while let Some(_) = case_records_iter.peek() {
            if !mismatch_tracker.iter().fold(false, |expr, (val_l, _)| (expr | val_l) ) & !first_run {
                next_case_record = case_records_iter.next();
                next_price_record = price_records_iter.next();
                next_calcs_record = calcs_records_iter.next();
                next_bal_record = bal_records_iter.next();
            } else {
                if mismatch_tracker[0].0 {
                    if mismatch_tracker[0].1 {
                        next_price_record = price_records_iter.next();
                    } else {
                        next_case_record = case_records_iter.next();
                    }
                }
                if mismatch_tracker[1].0 {
                    if mismatch_tracker[1].1 {
                        next_calcs_record = calcs_records_iter.next();
                    } else {
                        next_case_record = case_records_iter.next();
                    }
                }
                if mismatch_tracker[2].0 {
                    if mismatch_tracker[2].1 {
                        next_bal_record = bal_records_iter.next();
                    } else {
                        next_case_record = case_records_iter.next();
                    }
                }
            }
            first_run = false;
            mismatch_tracker = vec![(false, false), (false, false), (false, false)];

            if let (Some(next_price_record_unwrap), Some(next_calcs_record_unwrap), Some(next_bal_record_unwrap)) = (&next_price_record, &next_calcs_record, &next_bal_record) {
                if let Some(Ok(ref mut new_record)) = next_case_record {
                    let year = new_record.get(0).unwrap().to_string();
                    let quarter = new_record.get(1).unwrap().to_string();
                    let year_number = year.parse::<usize>().unwrap();
                    let quarter_number = quarter[1..=1].parse::<usize>().unwrap();

                    if let (Ok(next_price_record_unwrap_unwrap), Ok(next_calcs_record_unwrap_unwrap), Ok(next_bal_record_unwrap_unwrap)) = (next_price_record_unwrap, next_calcs_record_unwrap, next_bal_record_unwrap) {
                        // price
                        let price_year = &next_price_record_unwrap_unwrap.get(0).unwrap()[0..=3];
                        let price_month = &next_price_record_unwrap_unwrap.get(0).unwrap()[5..=6];
                        if price_year == year {
                            if !(((price_month == "03") & (quarter == "Q1")) | ((price_month == "06") & (quarter == "Q2")) | ((price_month == "09") & (quarter == "Q3")) | ((price_month == "12") & (quarter == "Q4"))) {
                                let price_quarter_number = price_month.parse::<usize>().unwrap() / 3;
                                mismatch_tracker[0] = (true, price_quarter_number >= quarter_number);
                                println!("Mismatch in price quarter. Price (Y, M): {:?}, New (Y, M): {:?}.", (price_year, price_month), (year, quarter));
                                continue;
                            }
                        } else {
                            let price_year_number = price_year.parse::<usize>().unwrap();
                            mismatch_tracker[0] = (true, price_year_number > year_number);
                            println!("Mismatch in price quarter. Price (Y, M): {:?}, New (Y, M): {:?}.", (price_year, price_month), (year, quarter));
                            continue;
                        }
                        // calcs
                        let calcs_year = next_calcs_record_unwrap_unwrap.get(0).unwrap();
                        let calcs_quarter = next_calcs_record_unwrap_unwrap.get(1).unwrap();
                        if calcs_year == year {
                            if calcs_quarter != quarter {
                                let calcs_quarter_number = calcs_quarter[1..=1].parse::<usize>().unwrap();
                                mismatch_tracker[1] = (true, calcs_quarter_number > quarter_number);
                                println!("Mismatch in calculations quarter. Calculation (Y, M): {:?}, New (Y, M): {:?}.", (calcs_year, calcs_quarter), (year, quarter));
                                continue;
                            }
                        } else {
                            let calcs_year_number = calcs_year.parse::<usize>().unwrap();
                            mismatch_tracker[1] = (true, calcs_year_number > year_number);
                            println!("Mismatch in calculations quarter. Calculation (Y, M): {:?}, New (Y, M): {:?}.", (calcs_year, calcs_quarter), (year, quarter));
                            continue;
                        }
                        // bal
                        let bal_field = next_bal_record_unwrap_unwrap.get(0).unwrap();
                        if bal_field.len() == 4 {
                            if &bal_field[0..=3] != year {
                                let bal_year_number = bal_field[0..=3].parse::<usize>().unwrap();
                                mismatch_tracker[2] = (true, bal_year_number > year_number);
                                println!("Mismatch in balance quarter. Balance (Y, M): {:?}, New (Y, M): {:?}.", bal_field, (year, quarter));
                                continue;
                            }
                        } else {
                            if &bal_field[1..=4] == year {
                                if &bal_field[8..=9] != quarter {
                                    let bal_quarter_number = bal_field[9..=9].parse::<usize>().unwrap();
                                    mismatch_tracker[2] = (true, bal_quarter_number > quarter_number);
                                    println!("Mismatch in balance quarter. Balance (Y, M): {:?}, New (Y, M): {:?}.", bal_field, (year, quarter));
                                    continue;
                                }
                            } else {
                                let bal_year_number = bal_field[1..=4].parse::<usize>().unwrap();
                                mismatch_tracker[2] = (true, bal_year_number > year_number);
                                println!("Mismatch in balance quarter. Balance (Y, M): {:?}, New (Y, M): {:?}.", bal_field, (year, quarter));
                                continue;
                            }
                        }
                        // If you've reached here, there was no mismatch.
                        drop_and_extend(new_record, next_price_record_unwrap_unwrap, 1);
                        drop_and_extend(new_record, next_calcs_record_unwrap_unwrap, 2);
                        drop_and_extend(new_record, next_bal_record_unwrap_unwrap, 1);
                    } else {
                        println!("Iterator Read Error");
                        continue;
                    }
                    // Write and remember the yr and quarter
                    let this_year = &new_record.get(0).unwrap().parse::<i64>().unwrap();
                    let this_quarter = &new_record.get(1).unwrap()[1..=1].parse::<i64>().unwrap();
                    if !written_once | ((last_year == *this_year) & (*this_quarter == last_quarter - 1)) | ((last_quarter == 1) & (*this_quarter == 4) & (*this_year == last_year - 1)) {
                        written_once = true;
                        last_year = *this_year;
                        last_quarter = *this_quarter;
                        if let Err(_) = unite.write_record(new_record.iter()) {
                            println!("WRITE ERROR.");
                        } else {
                            if let Err(_) = unite.flush() {
                                println!("FLUSH ERROR.");
                            }
                        }
                    } else {
                        println!("Continuity lost when attempting to write {:?} after {:?}.", (this_year, this_quarter), (last_year, last_quarter));
                        break;
                    }
                } else {
                    println!("Case iterator read error.");
                    break;
                }
            } else {
                println!("An iterator ended before the case iterator did.");
                break;
            }
        }
    }

    pub fn create_all_unites() {
        // Configure Path, open stock_names
        let mut path = current_dir().unwrap();
        path.pop(); path.push("test-data/stock_names.csv");
        let mut stock_names = Reader::from_path(&path).unwrap();

        // Pull out header, for each entry run a unite.
        if let Ok(stock_names_record) = stock_names.headers() {
            let stock_names_record_iter = stock_names_record.iter();
            for field in stock_names_record_iter {
                println!("Uniting {:?}...", field);
                unite_stock_csvs(field.to_string());
            }
        }
    }

    pub fn trim_and_sort() {
        // Path to unite folder
        let mut unite_folder = current_dir().unwrap();
        unite_folder.pop(); unite_folder.push("test-data/UnitedData");
        // Path to stock_names
        let mut path = current_dir().unwrap();
        path.pop(); path.push("test-data/stock_names.csv");
        // Populate vector of readers
        let mut stock_names = Reader::from_path(&path).unwrap();
        let mut file_readers = Vec::new();
        if let Ok(stock_names_record) = stock_names.headers() {
            let stock_names_record_iter = stock_names_record.iter();
            for name in stock_names_record_iter {
                let mut temp_str = name.to_string();
                temp_str.push_str("_unite.csv");
                unite_folder.push(temp_str);
                file_readers.push(Reader::from_path(&unite_folder).unwrap());
                unite_folder.pop();
            }
        }
        // Iterate over the comparitive header and make a vector of boolean acceptance of the headers
        let mut headers_to_keep = Vec::new();
        // Remove the first reader, it will be the comparitive source
        let mut file_readers_iter = file_readers.iter_mut();
        let header_reader = file_readers_iter.next();
        let mut file_readers = Vec::from_iter(file_readers_iter);

        let headers = header_reader.unwrap().headers().unwrap();
        'a : for field_to_find in headers {                                     // loop over prospective header
            'b : for mut reader in &mut file_readers {                          // loop over readers
                'c : for potential_field in reader.headers().unwrap().iter() {  // loop of elements of reader
                    if potential_field == field_to_find {
                        continue 'b;
                    } else {
                        continue 'c;
                    }
                }
                // Being here means field_to_find wasn't found in the current reader
                headers_to_keep.push(false);
                continue 'a;
            }
            // Being here means every reader contained field_to_find
            headers_to_keep.push(true);
        }
        // Create the new header and sort it
        let new_header = headers.iter().zip(headers_to_keep.iter()).filter_map(|(header, keep)| {
            if *keep {
                Some(header)
            } else {
                None
            }
        });
        let mut new_header_vec = Vec::from_iter(new_header);
        new_header_vec.sort();
        // Path to trim_unite folder
        let mut trim_unite_folder = current_dir().unwrap();
        trim_unite_folder.pop(); trim_unite_folder.push("test-data/TrimmedUnitedData");
        // Create the new files
        // Define the readers and writers
        if let Ok(stock_names_record) = stock_names.headers() {
            let stock_names_record_iter = stock_names_record.iter();
            // For each stock
            for name in stock_names_record_iter {
                // Open a writer
                let mut temp_str = name.to_string();
                temp_str.push_str("_unite_trim.csv");
                trim_unite_folder.push(temp_str);
                let mut writer = Writer::from_path(&trim_unite_folder).unwrap();
                trim_unite_folder.pop();
                // Open a reader
                let mut temp_str = name.to_string();
                temp_str.push_str("_unite.csv");
                unite_folder.push(temp_str);
                let mut reader = Reader::from_path(&unite_folder).unwrap();
                unite_folder.pop();
                // Get the header
                let mut indices = vec![None; Vec::from_iter(reader.headers().unwrap().iter()).len()];
                {
                    let old_header = Vec::from_iter(reader.headers().unwrap().iter());
                    // Build the indice list
                    'i : for i in 0..old_header.len() {
                        'j : for j in 0..new_header_vec.len() {
                            if old_header[i] == new_header_vec[j] {
                                indices[i] = Some(j);
                                continue 'i;
                            }
                        }
                        indices[i] = None;
                    }
                }
                // Push the new header
                writer.write_record(&new_header_vec);
                // Iterate over old rows
                for old_row_wrapped in reader.records() {
                    if let Ok(old_row) = old_row_wrapped {
                        let mut new_record = vec![""; new_header_vec.len()];
                        for i in 0..old_row.len() {
                            if let Some(index) = indices[i] {
                                new_record[index] = old_row.get(i).unwrap();
                            }
                        }
                        writer.write_record(new_record);
                    }
                }
            }
        }
    }
}
