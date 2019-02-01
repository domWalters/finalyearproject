#!/bin/bash

cd "$(dirname "$0")"
cd zip-files

ZIPNAME=`date +%Y-%m-%d`
ZIPNAME=${ZIPNAME}_data.zip

zip -r $ZIPNAME ../test-data/TrimmedUnitedData/
