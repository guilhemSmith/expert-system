input_files=`ls tests/input-files`;
input_dir="tests/input-files/";
output_dir="tests/expected-output/";

for fn in $input_files; do
	result=`./target/debug/expert-system $input_dir$fn`;
	echo $result;
done