year=$1
day=$(printf "%02d" $2)
cp scripts/day.rs $year/rust/solutions/day$day.rs
sed -i s:XX:$day: $year/rust/solutions/day$day.rs
