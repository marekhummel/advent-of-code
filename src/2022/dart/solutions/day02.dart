import 'package:collection/collection.dart';

import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final score = _parse(input)
      .map(
        (strategy) => switch (strategy) {
          ('A', 'X') => 1 + 3,
          ('A', 'Y') => 2 + 6,
          ('A', 'Z') => 3 + 0,
          ('B', 'X') => 1 + 0,
          ('B', 'Y') => 2 + 3,
          ('B', 'Z') => 3 + 6,
          ('C', 'X') => 1 + 6,
          ('C', 'Y') => 2 + 0,
          ('C', 'Z') => 3 + 3,
          _ => throw StateError('Invalid strategy'),
        },
      )
      .sum;

  return Result.integer(score);
}

Result part2(List<String> input, bool sample) {
  final score = _parse(input)
      .map(
        (strategy) => switch (strategy) {
          ('A', 'X') => 0 + 3,
          ('A', 'Y') => 3 + 1,
          ('A', 'Z') => 6 + 2,
          ('B', 'X') => 0 + 1,
          ('B', 'Y') => 3 + 2,
          ('B', 'Z') => 6 + 3,
          ('C', 'X') => 0 + 2,
          ('C', 'Y') => 3 + 3,
          ('C', 'Z') => 6 + 1,
          _ => throw StateError('Invalid strategy'),
        },
      )
      .sum;

  return Result.integer(score);
}

List<(String, String)> _parse(List<String> input) =>
    input.map((line) => (line[0], line[2])).toList();
