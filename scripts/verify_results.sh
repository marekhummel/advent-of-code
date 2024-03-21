# Check if solutions mentioned in rust files match with submitted solutions
cookie=$(cat scripts/aoc_cookie.txt)

failed=0
for year in $(seq 2023 2023)
do

    for day in $(seq 1 25)
    do
        html=$(curl "https://adventofcode.com/$year/day/$day" -H "cookie: session=$cookie" 2> /dev/null)
        submitted=($(echo "$html" | grep -Po "(?<=Your puzzle answer was <code>).*?(?=<\/code>)"))
        
        fday=$(printf "%02d" $day)
        code=$(cat "$year/rust/solutions/day$fday.rs")
        noted=($(echo "$code" | grep -Po "(?<=ProblemResult::).*(?=,)" | \
                cut -d'(' -f2 | cut -d')' -f1 | sed s/"\".to_string"// | sed s/"\""// ))
        echo -n "Check $year / $day:  "

    
        # echo "'${noted[@]}'"
        # continue

        if [ ${#submitted[@]} -ne 2 ] && [ $day -ne 25 ]; then
            echo "NO SUBMITTED RESULTS FOUND: '${submitted[@]}'"
            failed=$((failed+1))
            continue
        fi

        if [ ${#noted[@]} -ne 4 ]; then
            echo "NO CODED RESULTS FOUND: '${noted[@]}'"
            failed=$((failed+1))
            continue
        fi

        if [ "${submitted[0]}" != "${noted[1]}" ] || [ $day -ne 25 ] && [ "${submitted[1]}" != "${noted[3]}" ]; then
            echo "MISMATCH '${submitted[0]}' != '${noted[1]}' || '${submitted[1]}' != '${noted[3]}'"
            failed=$((failed+1))
            continue
        fi

        echo "OK"
    done

    echo 
done

echo "Verification complete: $failed checks failed."
