import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final sensors = _parse(input);
  final row = sample ? 10 : 2000000;
  final blocked = <Range>[];
  final beacons = <int>{};

  // Mark the sensor intervals overlapping the significant row
  for (final (sensor, beacon) in sensors) {
    final radius = sensor.manhattanDistance(beacon);
    final yDist = (row - sensor.y).abs();
    if (yDist > radius) continue;

    if (beacon.y == row) beacons.add(beacon.x);
    final halfWidth = radius - yDist;
    blocked.add(range(sensor.x - halfWidth, sensor.x + halfWidth));
  }

  // Merge overlapping intervals until no more can be combined
  while (true) {
    final newBlocked = <Range>[];
    final before = blocked.length;
    final merged = <int>{};
    for (var i = 0; i < blocked.length; i++) {
      for (var j = i + 1; j < blocked.length; j++) {
        if (merged.contains(i) || merged.contains(j)) continue;

        final a = blocked[i];
        final b = blocked[j];
        if (a.overlaps(b)) {
          newBlocked.add(a.span(b));
          merged.addAll([i, j]);
        }
      }
    }

    // Remove merged intervals and append their replacements
    final indices = merged.sortedDesc();
    for (final i in indices) {
      blocked.removeAt(i);
    }
    blocked.addAll(newBlocked);

    if (blocked.length == before) break;
  }

  // Count covered positions, excluding known beacons
  final count = blocked.map((interval) => interval.length).sum;

  return Result.integer(count - beacons.length);
}

Result part2(List<String> input, bool sample) {
  final sensors = _parse(input);
  final bound = sample ? 20 : 4000000;

  // Transform sensor diamonds into squares along the diagonal axes
  final ranges = sensors.map((pair) {
    final (sensor, beacon) = pair;
    final radius = sensor.manhattanDistance(beacon);
    final left = sensor.x - radius;
    final right = sensor.x + radius + 1;
    return ((left + sensor.y, left - sensor.y), (right + sensor.y, right - sensor.y));
  }).toList();

  // Reduce the grid to the mentioned diagonal coordinates
  final positions = ranges.expand((corners) => [corners.$1, corners.$2]).toList();
  final xx = positions.map((p) => p.x).toSet().sortedAsc();
  final yy = positions.map((p) => p.y).toSet().sortedAsc();
  final grid = Grid.create(xx.length - 1, yy.length - 1, false);
  final xxMap = {for (var i = 0; i < xx.length; i++) xx[i]: i};
  final yyMap = {for (var i = 0; i < yy.length; i++) yy[i]: i};

  // Mark the sensor areas in the compressed grid
  for (final (start, end) in ranges) {
    for (var y = yyMap[start.y]!; y < yyMap[end.y]!; y++) {
      for (var x = xxMap[start.x]!; x < xxMap[end.x]!; x++) {
        grid[y][x] = true;
      }
    }
  }

  // Find an uncovered area and convert back to ordinary coordinates
  for (var y = 0; y < grid.length; y++) {
    for (var x = 0; x < grid[y].length; x++) {
      if (!grid[y][x]) {
        final dx = xx[x];
        final dy = yy[y];
        final px = (dx + dy) ~/ 2;
        final py = (dx - dy) ~/ 2;
        if (range(0, bound).contains(px) && range(0, bound).contains(py)) {
          return Result.integer(px * 4000000 + py);
        }
      }
    }
  }

  throw StateError('No free area found');
}

List<(Index, Index)> _parse(List<String> input) {
  final pattern = RegExp(r'(?:x|y)=(-?\d+)');

  return input.map((line) {
    final values = pattern.allMatches(line).map((m) => int.parse(m[1]!)).toList();
    return ((values[0], values[1]), (values[2], values[3]));
  }).toList();
}
