cnt=0
total=0
for file in sample/*.tiger; do
    cargo run $file
    if [ $? -eq 0 ]; then cnt=`expr $cnt + 1`; fi
    total=`expr $total + 1`
done
echo sample-run: $cnt/$total tiger files passed
