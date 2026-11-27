import 'dart:math';

import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final cave = _parse(input);
  var sand = 0;
  while (!_produceSand(cave, true)) {
    sand++;
  }

  return Result.integer(sand);
}

Result part2(List<String> input, bool sample) {
  final cave = _parse(input);
  var sand = 1;
  while (!_produceSand(cave, false)) {
    sand++;
  }

  return Result.integer(sand);
}

List<List<String>> _parse(List<String> input) {
  final corners = input
      .map(
        (line) => line.split(' -> ').map((point) {
          final xy = point.split(',').map(int.parse).toList();
          return (xy[0], xy[1]);
        }).toList(),
      )
      .toList();
  final width = corners.flattened.map((p) => p.x).max + 1;
  final height = corners.flattened.map((p) => p.y).max + 2;

  // Allow enough width for the largest possible pyramid of sand
  final cave = Grid.create(max(width, 500 + height), height, '.');

  // Draw the rock structures
  for (final structure in corners) {
    for (var i = 0; i + 1 < structure.length; i++) {
      final start = structure[i];
      final end = structure[i + 1];
      if (start.x == end.x) {
        for (var y = min(start.y, end.y); y <= max(start.y, end.y); y++) {
          cave.setAt((start.x, y), '#');
        }
      } else if (start.y == end.y) {
        for (var x = min(start.x, end.x); x <= max(start.x, end.x); x++) {
          cave.setAt((x, start.y), '#');
        }
      }
    }
  }

  return cave;
}

bool _produceSand(List<List<String>> cave, bool abyss) {
  var position = (500, 0);

  while (true) {
    final below = position + Direction.south.offset;
    if (cave.containsIndex(below)) {
      final next = [
        below,
        below + Direction.west.offset,
        below + Direction.east.offset,
      ].firstWhereOrNull((candidate) => cave.at(candidate) == '.');
      if (next != null) {
        position = next;
        continue;
      }
    } else if (abyss) {
      return true;
    }

    // The grain rests; reaching the source is the part-two sentinel
    cave.setAt(position, 'o');
    return position == (500, 0);
  }
}
