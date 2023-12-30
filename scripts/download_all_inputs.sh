
cookie=$(cat scripts/aoc_cookie.txt)
for i in {1..25}
do
    day=$(printf "%02d" $i)
    curl "https://adventofcode.com/$1/day/$i/input" -H "cookie: session=$cookie" -o "$1/inputs/input$day.txt" 2>/dev/null
done
