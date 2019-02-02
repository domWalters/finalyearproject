#!/bin/bash

cd "$(dirname "$0")"
cp ../test-data/README.md ../test-data/TrimmedUnitedData/README.md

ZIPNAME=`date +%Y-%m-%d`
ZIPNAME=../zip-files/${ZIPNAME}_data.zip

zip -r $ZIPNAME ../test-data/TrimmedUnitedData/
rm ../test-data/TrimmedUnitedData/README.md
