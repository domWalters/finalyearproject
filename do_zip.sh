#!/bin/bash

cd ZipFiles

ZIPNAME=`date +%Y-%m-%d`
echo $ZIPNAME
ZIPNAME=${ZIPNAME}_data.zip
echo $ZIPNAME

zip -r $ZIPNAME ../test-data/TrimmedUnitedData/
