#!/bin/bash

echo "Creating necessary folders..."
./create_data_folders.sh
echo "Uniting data..."
./run_data_unite.sh
echo "Deleting extra data..."
./delete_extra_data.sh

