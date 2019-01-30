extern crate csv;

mod csv_reader_core;

fn main() {
    csv_reader_core::assemble_four_file_data();
    csv_reader_core::complex_reverse();
    csv_reader_core::create_all_unites();
    csv_reader_core::trim_and_sort();
}
