year=$1
day=$(printf "%02d" $2)
lang=$3

if [ $2 -gt 25 ]; then
    echo "Invalid day"
    exit 1
fi

if [ "$lang" != "rust" ] && [ "$lang" != "zig" ]; then
    echo "Invalid language (use 'rust' or 'zig'): '$lang'"
    exit 1
fi

if [ "$lang" = "rust" ] ; then
    cp scripts/templates/day_template.rs $year/rust/solutions/day$day.rs
    sed -i s:XX:$day: $year/rust/solutions/day$day.rs

    sed -i "$(($2+4)) a\use solutions::day$day;" $year/rust/main.rs 
    sed -i "$(($2*2+12)) a\        Box::new(day$day::Solution$day {})," $year/rust/main.rs 
    sed -i "$(($2*3+28)) a\    test_day!(day$day);" $year/rust/main.rs 

    if [ $2 -eq 1 ]; then 
        echo "pub mod day01;" > $year/rust/solutions/mod.rs
    else 
        sed -i "$(($2-1)) a\pub mod day$day;" $year/rust/solutions/mod.rs
    fi
elif [ "$lang" = "zig" ] ; then
    cp scripts/templates/day_template.zig $year/zig/solutions/day$day.zig

    sed -i "$(($2+2)) a\const day$day = @import(\"solutions/day$day.zig\");" $year/zig/main.zig 
    sed -i "$(($2*2+14)) a\        aoc_lib.solution.makeSolution(day$day)," $year/zig/main.zig 
fi

cookie=$(cat scripts/aoc_cookie.txt)
curl "https://adventofcode.com/$year/day/$2/input" -H "cookie: session=$cookie" -o "$year/inputs/input$day.txt" 2>/dev/null
touch $year/inputs/sample$day.txt