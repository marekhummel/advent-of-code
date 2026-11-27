import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/input.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final forest = input.grid().mapCells(int.parse);
  var visible = 0;

  for (final index in forest.indices) {
    final height = forest.at(index);

    // A tree is visible if any compass direction reaches the edge unobstructed
    final isVisible = Direction.values.any((direction) {
      var next = index + direction.offset;
      while (forest.containsIndex(next)) {
        if (forest.at(next) >= height) return false;
        next += direction.offset;
      }
      return true;
    });

    if (isVisible) visible++;
  }

  return Result.integer(visible);
}

Result part2(List<String> input, bool sample) {
  final forest = input.grid().mapCells(int.parse);

  final scores = forest.indices.map((index) {
    final ranges = Direction.values.map((direction) {
      var range = 0;
      var next = index + direction.offset;
      while (forest.containsIndex(next)) {
        range++;
        if (forest.at(next) >= forest.at(index)) break;
        next += direction.offset;
      }

      return range;
    });

    return ranges.product;
  });

  return Result.integer(scores.max);
}
