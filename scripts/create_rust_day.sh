year=$1
day=$(printf "%02d" $2)
cp scripts/day.rs $year/rust/solutions/day$day.rs
sed -i s:XX:$day: $year/rust/solutions/day$day.rs

sed -i "$(($2+2)) a\use solutions::day$day;" $year/rust/main.rs 
sed -i "$(($2*2+12)) a\        Box::new(day$day::Solution$day {})," $year/rust/main.rs 
sed -i "$(($2-1)) a\pub mod day$day;" $year/rust/solutions/mod.rs

touch $year/inputs/sample$day.txt
touch $year/inputs/input$day.txt

