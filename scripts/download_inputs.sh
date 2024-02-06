
cookie=$(cat scripts/aoc_cookie.txt)
if [ $# -lt 2 ]; then days=$(seq 1 25); else days=$(seq $2 $2); fi
for i in $days
do
    day=$(printf "%02d" $i)
    curl "https://adventofcode.com/$1/day/$i/input" -H "cookie: session=$cookie" -o "$1/inputs/input$day.txt" 2>/dev/null
done
