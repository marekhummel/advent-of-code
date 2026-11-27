import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/input.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  return Result.integer(_elfCalories(input).first);
}

Result part2(List<String> input, bool sample) {
  return Result.integer(_elfCalories(input).take(3).sum);
}

List<int> _elfCalories(List<String> input) {
  return input.blocks().map((group) => group.map(int.parse).sum).sortedDesc();
}
