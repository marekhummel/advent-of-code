use crate::solution::{ProblemInput, ProblemResult, Solution};

#[derive(Debug)]
struct Number {
    value: u32,
    row: usize,
    col_start: usize,
    col_end: usize,
}

#[derive(Debug)]
struct Symbol {
    value: char,
    row: usize,
    col: usize,
}

pub struct Solution03 {}

impl Solution03 {
    fn parse(&self, lines: ProblemInput) -> (Vec<Number>, Vec<Symbol>) {
        let mut numbers = vec![];
        let mut symbols = vec![];
        for (i, line) in lines.iter().enumerate() {
            let mut num_str = String::from("");
            let mut number_start = 0;
            for (j, ch) in line.chars().enumerate() {
                if ch.is_ascii_digit() {
                    if num_str.is_empty() {
                        number_start = j;
                    }
                    num_str.push(ch);
                } else {
                    if !num_str.is_empty() {
                        numbers.push(Number {
                            value: num_str.parse().unwrap(),
                            row: i,
                            col_start: number_start,
                            col_end: j - 1,
                        });
                        num_str.clear();
                    }

                    if ch != '.' && ch != '\n' {
                        symbols.push(Symbol {
                            value: ch,
                            row: i,
                            col: j,
                        })
                    }
                }
            }

            if !num_str.is_empty() {
                numbers.push(Number {
                    value: num_str.parse().unwrap(),
                    row: i,
                    col_start: number_start,
                    col_end: line.len() - 1,
                });
            }
        }

        (numbers, symbols)
    }

    fn is_part(&self, num: &Number, symbols: &Vec<Symbol>) -> bool {
        for sym in symbols {
            if sym.row == num.row && (sym.col + 1 == num.col_start || sym.col == num.col_end + 1) {
                return true;
            }
            if (sym.row + 1 == num.row || sym.row == num.row + 1)
                && (num.col_start <= sym.col + 1 && sym.col <= num.col_end + 1)
            {
                return true;
            }
        }

        false
    }

    fn gear_ratio(&self, sym: &Symbol, numbers: &Vec<Number>) -> u32 {
        if sym.value != '*' {
            return 0;
        }

        let mut part_numbers = vec![];
        for num in numbers {
            if sym.row == num.row && (sym.col + 1 == num.col_start || sym.col == num.col_end + 1) {
                part_numbers.push(num.value);
                continue;
            }
            if (sym.row + 1 == num.row || sym.row == num.row + 1)
                && (num.col_start <= sym.col + 1 && sym.col <= num.col_end + 1)
            {
                part_numbers.push(num.value);
                continue;
            }
        }

        match part_numbers.len() {
            2 => {
                return part_numbers.iter().product();
            }
            _ => 0,
        }
    }
}

impl Solution for Solution03 {
    fn get_day(&self) -> u8 {
        3
    }

    fn solve_version01(&self, input: ProblemInput) -> ProblemResult {
        let (numbers, symbols) = self.parse(input);
        numbers
            .iter()
            .filter(|n| self.is_part(n, &symbols))
            .map(|n| n.value)
            .sum::<u32>()
            .try_into()
            .unwrap()
    }

    fn solve_version02(&self, input: ProblemInput) -> ProblemResult {
        let (numbers, symbols) = self.parse(input);
        symbols
            .iter()
            .map(|s| self.gear_ratio(s, &numbers))
            .sum::<u32>()
            .try_into()
            .unwrap()
    }
}
