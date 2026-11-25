import '../../lib/dart/runner.dart';
import 'solutions/day01.dart' as day01;

// aoc:imports

void main(List<String> args) {
  run(2022, {
    1: Solution(part1: day01.part1, part2: day01.part2),
    // aoc:solutions
  }, args);
}
