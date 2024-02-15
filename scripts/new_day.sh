year=$1
day=$(printf "%02d" $2)
cp scripts/templates/day_template.rs $year/rust/solutions/day$day.rs
sed -i s:XX:$day: $year/rust/solutions/day$day.rs

sed -i "$(($2+2)) a\use solutions::day$day;" $year/rust/main.rs 
sed -i "$(($2*2+12)) a\        Box::new(day$day::Solution$day {})," $year/rust/main.rs 

if [ $2 -eq 1 ]; then 
    echo "pub mod day01;" > $year/rust/solutions/mod.rs
else 
    sed -i "$(($2-1)) a\pub mod day$day;" $year/rust/solutions/mod.rs
fi

cookie=$(cat scripts/aoc_cookie.txt)
curl "https://adventofcode.com/$year/day/$2/input" -H "cookie: session=$cookie" -o "$year/inputs/input$day.txt" 2>/dev/null
touch $year/inputs/sample$day.txt