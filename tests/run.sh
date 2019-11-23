#! /bin/bash
input_files=`ls tests/input-files`;
input_dir="tests/input-files/";
output_dir="tests/expected-output/";
count_err=0;

for fn in $input_files; do
	result=`./target/debug/expert-system $input_dir$fn`;
	if [ $? -eq 0 ]
	then
		cmp=`echo "$result" | diff -y $output_dir$fn -`;
		if [ $? -eq 0 ]
		then
			echo "$fn: OK";
		else
			echo "$fn: KO";
			echo $cmp;
			count_err=$((count_err+1));
		fi
	else
		echo "$fn failed with an error:\n$result";
		count_err=$((count_err+1));
	fi
	echo " ---";
done
if [ $count_err -eq 0 ]
then
	echo "All tests succeed.";
	exit 0;
else
	echo "$count_err test(s) failed.";
	exit 1;
fi