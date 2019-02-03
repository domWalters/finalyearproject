use std::{iter::FromIterator, env::*, fs::*};
use csv::{Writer, Reader, StringRecord};

pub fn assemble_four_file_data() {
    // Configure paths
    let mut python = current_dir().unwrap();
    python.pop(); python.push("test-data/PythonData");
    let mut four = current_dir().unwrap();
    four.pop(); four.push("test-data/FourFileData/arbitrary_trash.csv");
    // Open Directory, pull out the first 4 things.
    let mut files: Vec<_> = read_dir(python).unwrap().map(|r| r.unwrap()).collect();
    files.sort_by_key(|dir| dir.file_name());
    let mut files_iter = files.iter();

    let mut first;                                      // first loop iteration populates this
    let mut second = files_iter.next().unwrap();
    let mut third = files_iter.next().unwrap();
    let mut fourth = files_iter.next().unwrap();

    for next_file in files_iter {

        first = second;
        second = third;
        third = fourth;
        fourth = next_file;

        let first_file_name = first.file_name().into_string().unwrap();
        let second_file_name = second.file_name().into_string().unwrap();
        let third_file_name = third.file_name().into_string().unwrap();
        let fourth_file_name = fourth.file_name().into_string().unwrap();

        let first_stock_name = first_file_name.split('_').next().unwrap();
        let second_stock_name = second_file_name.split('_').next().unwrap();
        let third_stock_name = third_file_name.split('_').next().unwrap();
        let fourth_stock_name = fourth_file_name.split('_').next().unwrap();

        if (first_stock_name == second_stock_name) & (first_stock_name == third_stock_name) & (first_stock_name == fourth_stock_name) {
            // We have 4 file paths, copy them.
            four.set_file_name(first.file_name());
            if let Err(err) = copy(first.path(), &four) {
                println!("{:?}", err);
            }
            four.set_file_name(second.file_name());
            if let Err(err) = copy(second.path(), &four) {
                println!("{:?}", err);
            }
            four.set_file_name(third.file_name());
            if let Err(err) = copy(third.path(), &four) {
                println!("{:?}", err);
            }
            four.set_file_name(fourth.file_name());
            if let Err(err) = copy(fourth.path(), &four) {
                println!("{:?}", err);
            }
        }
    }
}

pub fn complex_reverse() {
    // Create necessary file paths
    let mut four = current_dir().unwrap();
    four.pop(); four.push("test-data/FourFileData");
    let mut four_rev = current_dir().unwrap();
    four_rev.pop(); four_rev.push("test-data/FourFileDataRev/arbitrary_trash.csv");
    // Open directory to FourFileData
    let mut files: Vec<_> = read_dir(four).unwrap().map(|r| r.unwrap()).collect();
    files.sort_by_key(|dir| dir.file_name());
    let files_iter = files.iter();
    for file in files_iter {
        // Create Reader/Writer
        let mut reader = Reader::from_path(file.path()).unwrap();
        four_rev.set_file_name(file.file_name());
        let mut writer = Writer::from_path(&four_rev).unwrap();
        // Push the header.
        if let Err(_) = writer.write_record(reader.headers().unwrap()) {
            println!("WRITE ERROR WITH HEADERS.");
            std::process::exit(1);
        } else {
            if let Err(_) = writer.flush() {
                println!("FLUSH ERROR.");
            }
        }
        let mut records_vec = reader.records().map(|record| record.unwrap()).collect::<Vec<_>>();
        if file.file_name().into_string().unwrap().contains("price") {
            four_rev.set_file_name(file.file_name());
            if let Err(err) = copy(file.path(), &four_rev) {
                println!("{:?}", err);
            }
        } else {
            if file.file_name().into_string().unwrap().contains("balance") {
                records_vec.sort_by_key(|record| {
                    let mut field = record.get(0).unwrap().to_string();
                    if field.len() == 4 {
                        field.push_str("Q4");
                    } else {
                        let mut field_new = field[1..=4].to_string();
                        field_new.push_str(&field[7..=8]);
                        field = field_new;
                    }
                    field
                });
            }
            for record in records_vec.iter().rev() {
                if let Err(_) = writer.write_record(record) {
                    println!("WRITE ERROR WITH RECORD.");
                    std::process::exit(1);
                } else {
                    if let Err(_) = writer.flush() {
                        println!("FLUSH ERROR.");
                    }
                }
            }
        }
    }
}

pub fn create_all_unites() {
    // Configure Path
    let mut four = current_dir().unwrap();
    four.pop(); four.push("test-data/FourFileDataRev");

    let mut files: Vec<_> = read_dir(four).unwrap().map(|r| r.unwrap()).collect();
    files.sort_by_key(|dir| dir.file_name());
    let mut files_iter = files.iter().peekable();
    while let Some(_) = files_iter.peek() {
        let first = files_iter.next().unwrap();
        files_iter.next(); files_iter.next(); files_iter.next();

        let first_file_name = first.file_name().into_string().unwrap();
        let first_stock_name = first_file_name.split('_').next().unwrap();

        println!("Uniting {:?}...", first_stock_name);
        unite_stock_csvs(first_stock_name.to_string());
    }
}

fn unite_stock_csvs(stock_string: String) {
    // Configure Path
    let mut path = current_dir().unwrap();
    path.pop(); path.push("test-data/FourFileDataRev/arbitrary_trash.csv");

    // Open all 4 file readers
    let mut bal = stock_string.clone();
    bal.push_str(&"_fudamentals_balance.csv".to_string());
    path.set_file_name(bal);
    let mut balance = Reader::from_path(&path).unwrap();

    let mut calcs = stock_string.clone();
    calcs.push_str(&"_fudamentals_calculations.csv".to_string());
    path.set_file_name(calcs);
    let mut calculations = Reader::from_path(&path).unwrap();

    let mut case = stock_string.clone();
    case.push_str(&"_fudamentals_caseflow.csv".to_string());
    path.set_file_name(case);
    let mut caseflow = Reader::from_path(&path).unwrap();

    let mut pri = stock_string.clone();
    pri.push_str(&"_price.csv".to_string());
    path.set_file_name(pri);
    let mut price = Reader::from_path(&path).unwrap();

    // Open writer
    let mut uni = stock_string.clone();
    uni.push_str(&"_unite.csv".to_string());
    path.pop(); path.pop(); path.push("UnitedData"); path.push(uni);
    let mut unite = Writer::from_path(&path).unwrap();

    // Construct new header
    {
        let case_headers_iter = caseflow.headers().unwrap().iter();
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

    let mut next_case_record_wrapped = case_records_iter.next();
    let mut next_price_record_wrapped = price_records_iter.next();
    let mut next_calcs_record_wrapped = calcs_records_iter.next();
    let mut next_bal_record = bal_records_iter.next();

    let mut first_run = true;
    let mut written_once = false;
    let mut last_year = 0;
    let mut last_quarter = 0;
    let mut mismatch_tracker = vec![(false, false), (false, false), (false, false)];
    // {(price mismatch?, price > case?),(calcs mismatch?, calcs > case?),(bal mismatch?, bal > case?)}

    // Iterate through rows in caseflow
    while let Some(_) = case_records_iter.peek() {
        if !mismatch_tracker.iter().fold(false, | acc, (val_l, _) | (acc | val_l) ) & !first_run {
            next_case_record_wrapped = case_records_iter.next();
            next_price_record_wrapped = price_records_iter.next();
            next_calcs_record_wrapped = calcs_records_iter.next();
            next_bal_record = bal_records_iter.next();
        } else {
            match mismatch_tracker[0] {
                (true, true) => next_price_record_wrapped = price_records_iter.next(),
                (true, false) => next_case_record_wrapped = case_records_iter.next(),
                _ => (),
            }
            match mismatch_tracker[1] {
                (true, true) => next_calcs_record_wrapped = calcs_records_iter.next(),
                (true, false) => next_case_record_wrapped = case_records_iter.next(),
                _ => (),
            }
            match mismatch_tracker[2] {
                (true, true) => next_bal_record = bal_records_iter.next(),
                (true, false) => next_case_record_wrapped = case_records_iter.next(),
                _ => (),
            }
        }
        first_run = false;
        mismatch_tracker = vec![(false, false), (false, false), (false, false)];

        if let (Some(Ok(next_price_record)), Some(Ok(next_calcs_record)), Some(Ok(next_bal_record))) = (&next_price_record_wrapped, &next_calcs_record_wrapped, &next_bal_record) {
            if let Some(Ok(ref mut new_record)) = next_case_record_wrapped {
                let year = new_record.get(0).unwrap().to_string();
                let quarter = new_record.get(1).unwrap().to_string();
                let year_number = year.parse::<usize>().unwrap();
                let quarter_number = quarter[1..=1].parse::<usize>().unwrap();

                // price
                let price_year = &next_price_record.get(0).unwrap()[0..=3];
                let price_month = &next_price_record.get(0).unwrap()[5..=6];
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
                let calcs_year = next_calcs_record.get(0).unwrap();
                let calcs_quarter = next_calcs_record.get(1).unwrap();
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
                let bal_field = next_bal_record.get(0).unwrap();
                if bal_field.len() == 4 {
                    if &bal_field[0..=3] != year {
                        let bal_year_number = bal_field[0..=3].parse::<usize>().unwrap();
                        mismatch_tracker[2] = (true, bal_year_number > year_number);
                        println!("Mismatch in balance quarter. Balance (Y, M): {:?}, New (Y, M): {:?}.", bal_field, (year, quarter));
                        continue;
                    }
                } else {
                    if &bal_field[1..=4] == year {
                        if &bal_field[7..=8] != quarter {
                            let bal_quarter_number = bal_field[8..=8].parse::<usize>().unwrap();
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
                drop_and_extend(new_record, next_price_record, 1);
                drop_and_extend(new_record, next_calcs_record, 2);
                drop_and_extend(new_record, next_bal_record, 1);
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

fn drop_and_extend(new_record: &mut StringRecord, old_record: &StringRecord, drop_count: usize) {
    let mut iterator = old_record.iter();
    for _i in 0..drop_count {
        iterator.next();
    }
    new_record.extend(iterator);
}

pub fn trim_and_sort() {
    // Path to unite folder
    let mut unite_folder = current_dir().unwrap();
    unite_folder.pop(); unite_folder.push("test-data/UnitedData");
    // Files list
    let files: Vec<_> = read_dir(unite_folder).unwrap().map(|r| r.unwrap()).collect();
    let files_iter = files.iter();
    // Populate vector of readers
    let mut file_readers = Vec::new();
    for file in files_iter {
        file_readers.push(Reader::from_path(file.path()).unwrap());
    }
    // Iterate over the comparitive header and make a vector of boolean acceptance of the headers
    let mut headers_to_keep = Vec::new();
    // Remove the first reader, it will be the comparitive source
    let mut file_readers_iter = file_readers.iter_mut();
    let header_reader = file_readers_iter.next();
    let mut file_readers = Vec::from_iter(file_readers_iter);

    let headers = header_reader.unwrap().headers().unwrap();
    'a : for field_to_find in headers {                                     // loop over prospective header
        'b : for reader in &mut file_readers {                              // loop over readers
            'c : for potential_field in reader.headers().unwrap().iter() {  // loop over elements of reader
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
    // Define the readers and writers
    let files_iter = files.iter();
    for file in files_iter {
        // Open a writer
        let mut temp_str = file.file_name().into_string().unwrap().split('_').next().unwrap().to_string();
        temp_str.push_str("_unite_trim.csv");
        trim_unite_folder.push(temp_str);
        let mut writer = Writer::from_path(&trim_unite_folder).unwrap();
        trim_unite_folder.pop();
        // Open a reader
        let mut reader = Reader::from_path(file.path()).unwrap();
        // Get the header
        let mut indices = vec![None; Vec::from_iter(reader.headers().unwrap().iter()).len()];
        {
            // Build the indice list
            'i : for (indice, old_field) in indices.iter_mut().zip(reader.headers().unwrap().iter()) {
                'j : for (j, new_field) in new_header_vec.iter().enumerate() {
                    if old_field == *new_field {
                        *indice = Some(j);
                        continue 'i;
                    }
                }
                *indice = None;
            }
        }
        // Push the new header
        if let Err(err) = writer.write_record(&new_header_vec) {
            println!("{:?}", err);
            panic!("Error when writing csv header.");
        }
        // Iterate over old rows
        for old_row_wrapped in reader.records() {
            if let Ok(old_row) = old_row_wrapped {
                let mut new_record = vec![""; new_header_vec.len()];
                for (old_field, indice) in old_row.iter().zip(indices.iter()) {
                    if let Some(index) = *indice {
                        new_record[index] = old_field;
                    }
                }
                if let Err(err) = writer.write_record(new_record) {
                    println!("{:?}", err);
                    panic!("Error when writing csv record.");
                }
            }
        }
    }
}
