pub mod csv_reader {

    use std::iter::FromIterator;
    use csv::{Reader, StringRecord};
    use std::fs::File;
    use std::error::Error;

    pub fn load_file(file: String) -> Reader<File> {
        let mut parent: String = "/home/dominic/Documents/uni/yr4/finalyearproject/test-data/Data/".to_string().to_owned();
        parent.push_str(&file.to_owned());
        Reader::from_path(parent).unwrap()
    }

    pub fn unite_csv(mut price: Reader<File>, mut calcs: Reader<File>, mut case: Reader<File>, mut bal: Reader<File>) -> Result<(), Box<Error>> {
        let mut wtr = csv::Writer::from_path("/home/dominic/Documents/uni/yr4/finalyearproject/test-data/Data/unite_csv.csv").unwrap();
        {
            let mut case_headers_iter = case.headers().unwrap().iter();
            let mut price_headers_iter = price.headers().unwrap().iter();
            price_headers_iter.next();
            let mut calcs_headers_iter = calcs.headers().unwrap().iter();
            calcs_headers_iter.next();
            calcs_headers_iter.next();
            let mut bal_headers_iter = bal.headers().unwrap().iter();
            bal_headers_iter.next();

            let mut new_headers = StringRecord::from_iter(case_headers_iter);
            new_headers.extend(price_headers_iter);
            new_headers.extend(calcs_headers_iter);
            new_headers.extend(bal_headers_iter);

            wtr.write_record(&new_headers)?;
        }

        let mut case_records_iter = case.records();     // first 2 columns
        let mut price_records_iter = price.records();    // hard one, needs more parse
        let mut calcs_records_iter = calcs.records();    // first 2 columns
        let mut bal_records_iter = bal.records();

        let mut first_run = true;
        let mut mismatch_tracker = vec![false, false, false];
        let mut next_case_record = case_records_iter.next();
        let mut next_price_record = price_records_iter.next();
        let mut next_calcs_record = calcs_records_iter.next();
        let mut next_bal_record = bal_records_iter.next();

        let mut case_records_peek_iter = case_records_iter.peekable();

        'core: while let Some(_) = case_records_peek_iter.peek() {
            println!("{:?}", mismatch_tracker);
            if !mismatch_tracker.iter().fold(false, |expr, val| (expr | val) ) & !first_run {
                next_case_record = case_records_peek_iter.next();
                next_price_record = price_records_iter.next();
                next_calcs_record = calcs_records_iter.next();
                next_bal_record = bal_records_iter.next();
            } else {
                if mismatch_tracker[0] {
                    next_price_record = price_records_iter.next();  // First element in form "YYYY-MM-DD"
                }
                if mismatch_tracker[1] {
                    next_calcs_record = calcs_records_iter.next();  // First 2 elements in same form as case
                }
                if mismatch_tracker[2] {
                    next_bal_record = bal_records_iter.next();      // First element in form "(YYYY, 'QN')"
                }
            }
            mismatch_tracker = vec![false, false, false];
            first_run = false;

            if let (Some(next_price_record_unwrap), Some(next_calcs_record_unwrap), Some(next_bal_record_unwrap)) =
                                                                (&next_price_record, &next_calcs_record, &next_bal_record) {
                if let Some(Ok(ref mut new_record)) = next_case_record {
                    // price
                    if let Ok(next_price_record_unwrap_unwrap) = next_price_record_unwrap {
                        let field = next_price_record_unwrap_unwrap.get(0).unwrap();
                        if &field[0..=3] == new_record.get(0).unwrap() {
                            if ((&field[5..=6] == "03") & (new_record.get(1).unwrap() == "Q1")) |
                                    ((&field[5..=6] == "06") & (new_record.get(1).unwrap() == "Q2")) |
                                    ((&field[5..=6] == "09") & (new_record.get(1).unwrap() == "Q3")) |
                                    ((&field[5..=6] == "12") & (new_record.get(1).unwrap() == "Q4")) {
                                let mut next_price_record_unwrap_unwrap_iter = next_price_record_unwrap_unwrap.iter();
                                next_price_record_unwrap_unwrap_iter.next();
                                new_record.extend(next_price_record_unwrap_unwrap_iter);
                            } else {
                                mismatch_tracker[0] = true;
                                println!("Mismatch in price quarter. Field: {:?}, Rec: {:?}.", field, new_record.get(1));
                                continue 'core;
                            }
                        } else {
                            mismatch_tracker[0] = true;
                            println!("Mismatch in price quarter. Field: {:?}, Rec: {:?}.", field, new_record.get(0));
                            continue 'core;
                        }
                    } else {
                        println!("Price Error");
                        continue 'core;
                    }
                    // calcs
                    if let Ok(next_calcs_record_unwrap_unwrap) = next_calcs_record_unwrap {
                        if (next_calcs_record_unwrap_unwrap.get(0).unwrap() == new_record.get(0).unwrap()) & (next_calcs_record_unwrap_unwrap.get(1).unwrap() == new_record.get(1).unwrap()) {
                            let mut next_calcs_record_unwrap_unwrap_iter = next_calcs_record_unwrap_unwrap.iter();
                            next_calcs_record_unwrap_unwrap_iter.next();
                            next_calcs_record_unwrap_unwrap_iter.next();
                            new_record.extend(next_calcs_record_unwrap_unwrap_iter);
                        } else {
                            mismatch_tracker[1] = true;
                            println!("Mismatch in calculations quarter. Field: {:?}, Rec: {:?}.", (next_calcs_record_unwrap_unwrap.get(0), next_calcs_record_unwrap_unwrap.get(1)), (new_record.get(0), new_record.get(1)));
                            continue 'core;
                        }
                    } else {
                        println!("Calculations Error");
                        continue 'core;
                    }
                    // bal
                    if let Ok(next_bal_record_unwrap_unwrap) = next_bal_record_unwrap {
                        let field = next_bal_record_unwrap_unwrap.get(0).unwrap();
                        if field.len() == 4 {
                            if &field[0..=3] == new_record.get(0).unwrap() {
                                let mut next_bal_record_unwrap_unwrap_iter = next_bal_record_unwrap_unwrap.iter();
                                next_bal_record_unwrap_unwrap_iter.next();
                                new_record.extend(next_bal_record_unwrap_unwrap_iter);
                            }
                        } else {
                            if (&field[1..=4] == new_record.get(0).unwrap()) & (&field[8..=9] == new_record.get(1).unwrap()) {
                                let mut next_bal_record_unwrap_unwrap_iter = next_bal_record_unwrap_unwrap.iter();
                                next_bal_record_unwrap_unwrap_iter.next();
                                new_record.extend(next_bal_record_unwrap_unwrap_iter);
                            } else {
                                mismatch_tracker[2] = true;
                                println!("Mismatch in balance quarter. Field: {:?}, Rec: {:?}.", field, (new_record.get(0), new_record.get(1)));
                                continue 'core;
                            }
                        }
                    } else {
                        println!("Balance Error");
                        continue 'core;
                    }
                    // Write
                    println!("{:?}", new_record);
                    wtr.write_record(new_record.iter())?;
                } else {
                    println!("Case read error.");
                    break 'core;
                }
            } else {
                println!("AN ITERATOR ENDED");
                break 'core;
            }
        }
        Ok(())
    }

}
