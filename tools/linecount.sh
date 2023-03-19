#!/bin/bash

output=linecount.txt
echo "Number of source code lines:" | tee $output
wc -l ../src/* | tee -a $output
