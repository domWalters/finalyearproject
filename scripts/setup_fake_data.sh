#!/bin/bash

cd "$(dirname "$0")"

cd ../test-data/TrimmedUnitedData

rm *.csv

cd ../../data_generator/

cargo run $1 $2 $3
