#!/bin/bash
output=wordcount.txt
echo "Number of source code lines:" | tee $output
wc -l ../src/* | tee -a $output
