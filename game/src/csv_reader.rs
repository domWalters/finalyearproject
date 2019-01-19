pub mod csv_reader {

    use std::{
        iter::FromIterator,
        fs::File,
        error::Error,
        env::*,
        path::PathBuf };
    use csv::{Writer, Reader, StringRecord};

    fn drop_and_extend(new_record: &mut StringRecord, old_record: &StringRecord, drop_count: usize) {
        let mut iterator = old_record.iter();
        for _i in 0..drop_count {
            iterator.next();
        }
        new_record.extend(iterator);
    }

    pub fn unite_stock_csvs(stock_string: String) {
        let mut stock_name = stock_string.clone();

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

        let mut price = stock_name.clone();
        price.push_str(&"_price.csv".to_string());
        path.push(price);
        let mut price = Reader::from_path(&path).unwrap();
        path.pop();

        // Open writer
        let mut uni = stock_name.clone();
        uni.push_str(&"_unite.csv".to_string());
        path.push(uni);
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

            unite.write_record(&new_headers);
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
        let mut mismatch_tracker = vec![false, false, false];

        // Iterate through rows in caseflow
        while let Some(_) = case_records_iter.peek() {
            if !mismatch_tracker.iter().fold(false, |expr, val| (expr | val) ) & !first_run {
                next_case_record = case_records_iter.next();
                next_price_record = price_records_iter.next();
                next_calcs_record = calcs_records_iter.next();
                next_bal_record = bal_records_iter.next();
            } else {
                if mismatch_tracker[0] {
                    next_price_record = price_records_iter.next();
                }
                if mismatch_tracker[1] {
                    next_calcs_record = calcs_records_iter.next();
                }
                if mismatch_tracker[2] {
                    next_bal_record = bal_records_iter.next();
                }
            }
            mismatch_tracker = vec![false, false, false];
            first_run = false;

            if let (Some(next_price_record_unwrap), Some(next_calcs_record_unwrap), Some(next_bal_record_unwrap)) =
                                                                (&next_price_record, &next_calcs_record, &next_bal_record) {
                if let Some(Ok(ref mut new_record)) = next_case_record {
                    let year = new_record.get(0).unwrap().to_string();
                    let quarter = new_record.get(1).unwrap().to_string();
                    // price
                    if let Ok(next_price_record_unwrap_unwrap) = next_price_record_unwrap {
                        let price_year = &next_price_record_unwrap_unwrap.get(0).unwrap()[0..=3];
                        let price_month = &next_price_record_unwrap_unwrap.get(0).unwrap()[5..=6];
                        if price_year == year {
                            if ((price_month == "03") & (quarter == "Q1")) |
                                    ((price_month == "06") & (quarter == "Q2")) |
                                    ((price_month == "09") & (quarter == "Q3")) |
                                    ((price_month == "12") & (quarter == "Q4")) {
                                drop_and_extend(new_record, next_price_record_unwrap_unwrap, 1);
                            } else {
                                mismatch_tracker[0] = true;
                                println!("Mismatch in price quarter. Price (Y, M): {:?}, New (Y, M): {:?}.", (price_year, price_month), (year, quarter));
                                continue;
                            }
                        } else {
                            mismatch_tracker[0] = true;
                            println!("Mismatch in price quarter. Price (Y, M): {:?}, New (Y, M): {:?}.", (price_year, price_month), (year, quarter));
                            continue;
                        }
                    } else {
                        println!("Price Iterator Read Error");
                        continue;
                    }
                    // calcs
                    if let Ok(next_calcs_record_unwrap_unwrap) = next_calcs_record_unwrap {
                        let calcs_year = next_calcs_record_unwrap_unwrap.get(0).unwrap();
                        let calcs_month = next_calcs_record_unwrap_unwrap.get(1).unwrap();
                        if (calcs_year == year) & (calcs_month == quarter) {
                            drop_and_extend(new_record, next_calcs_record_unwrap_unwrap, 2);
                        } else {
                            mismatch_tracker[1] = true;
                            println!("Mismatch in calculations quarter. Calculation (Y, M): {:?}, New (Y, M): {:?}.", (calcs_year, calcs_month), (year, quarter));
                            continue;
                        }
                    } else {
                        println!("Calculations Iterator Read Error");
                        continue;
                    }
                    // bal
                    if let Ok(next_bal_record_unwrap_unwrap) = next_bal_record_unwrap {
                        let bal_field = next_bal_record_unwrap_unwrap.get(0).unwrap();
                        if bal_field.len() == 4 {
                            if &bal_field[0..=3] == year {
                                drop_and_extend(new_record, next_bal_record_unwrap_unwrap, 1);
                            }
                        } else {
                            if (&bal_field[1..=4] == year) & (&bal_field[8..=9] == quarter) {
                                drop_and_extend(new_record, next_bal_record_unwrap_unwrap, 1);
                            } else {
                                mismatch_tracker[2] = true;
                                println!("Mismatch in balance quarter. Balance (Y, M): {:?}, New (Y, M): {:?}.", bal_field, (year, quarter));
                                continue;
                            }
                        }
                    } else {
                        println!("Balance Iterator Read Error");
                        continue;
                    }
                    // Write
                    unite.write_record(new_record.iter());
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

}
