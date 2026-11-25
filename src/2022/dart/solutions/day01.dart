import 'package:collection/collection.dart';

import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  return Result.integer(_elfCalories(input).first);
}

Result part2(List<String> input, bool sample) {
  return Result.integer(_elfCalories(input).take(3).sum);
}

List<int> _elfCalories(List<String> input) {
  return input
      .splitAfter((line) => line.isEmpty)
      .map((group) => group.where((line) => line.isNotEmpty).map(int.parse).sum)
      .sorted((a, b) => b.compareTo(a));
}
