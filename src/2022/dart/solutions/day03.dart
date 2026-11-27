import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final rucksacks = input.map(
    (line) => (
      line.substring(0, line.length ~/ 2).split('').toSet(),
      line.substring(line.length ~/ 2).split('').toSet(),
    ),
  );

  final failed = rucksacks.map((sacks) => sacks.$1.intersection(sacks.$2).first);

  return Result.integer(failed.map(_priority).sum);
}

Result part2(List<String> input, bool sample) {
  final rucksacks = input.map((line) => line.split('').toSet()).toList();

  final badges = rucksacks
      .slices(3)
      .where((group) => group.length == 3)
      .map((group) => group.reduce((a, b) => a.intersection(b)).first);

  return Result.integer(badges.map(_priority).sum);
}

int _priority(String item) {
  final code = item.codeUnitAt(0);

  return range(97, 122).contains(code) ? code - 97 + 1 : code - 65 + 27;
}
