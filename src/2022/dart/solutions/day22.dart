import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/input.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final (map, path) = _parse(input);

  // Move to the start
  var idx = (map[0].indexOf('.'), 0);
  var direction = Direction.east;

  // Follow the path
  for (final step in path) {
    final tiles = int.tryParse(step);
    if (tiles == null) {
      direction = step == 'L' ? direction.left : direction.right;
      continue;
    }

    var moves = 0;
    var lastValid = idx;
    while (moves < tiles) {
      final (dx, dy) = direction.offset;
      final next = ((idx.x + dx) % map[0].length, (idx.y + dy) % map.length);
      switch (map.at(next)) {
        case '.':
          idx = next;
          lastValid = idx;
          moves++;
        case ' ':
          idx = next;
        case '#':
          idx = lastValid;
          moves = tiles;
        default:
          throw StateError('Invalid map tile');
      }
    }
  }

  // Compute the password
  return Result.integer(1000 * (idx.y + 1) + 4 * (idx.x + 1) + (direction.index + 3) % 4);
}

Result part2(List<String> input, bool sample) {
  final (wholeMap, path) = _parse(input);
  final size = sample ? 4 : 50;
  final maps = _splitGrids(wholeMap, size, sample);

  // Move to the start
  var mapIndex = 0;
  var direction = Direction.east;
  var idx = (0, 0);

  // Follow the path
  for (final step in path) {
    final tiles = int.tryParse(step);
    if (tiles == null) {
      direction = step == 'L' ? direction.left : direction.right;
      continue;
    }

    for (var tile = 0; tile < tiles; tile++) {
      final next = idx + direction.offset;
      final (int, Index, Direction) transition;
      if (next.isInside(size, size)) {
        transition = (mapIndex, next, direction);
      } else {
        transition = _transition(mapIndex, idx, direction, size - 1, sample);
      }

      final (nextMap, nextIndex, nextDirection) = transition;

      // Hit a wall: retain the previous face, position, and direction
      if (maps[nextMap].at(nextIndex) == '#') break;

      mapIndex = nextMap;
      idx = nextIndex;
      direction = nextDirection;
    }
  }

  final (ox, oy) = _offsets(sample)[mapIndex];

  // Compute the password
  return Result.integer(
    1000 * (size * oy + idx.y + 1) + 4 * (size * ox + idx.x + 1) + (direction.index + 3) % 4,
  );
}

(int, Index, Direction) _transition(
  int face,
  Index idx,
  Direction direction,
  int last,
  bool sample,
) {
  final (x, y) = idx;

  // The Rust sample/real cube-net transitions are preserved exactly
  if (sample) {
    return switch ((face, direction)) {
      (0, Direction.north) => (1, (last - x, 0), Direction.south),
      (0, Direction.east) => (5, (last, last - y), Direction.west),
      (0, Direction.south) => (3, (x, 0), Direction.south),
      (0, Direction.west) => (2, (y, 0), Direction.south),
      (1, Direction.north) => (0, (last - x, 0), Direction.south),
      (1, Direction.east) => (2, (0, y), Direction.east),
      (1, Direction.south) => (4, (last - x, last), Direction.north),
      (1, Direction.west) => (5, (last - y, last), Direction.north),
      (2, Direction.north) => (0, (0, x), Direction.east),
      (2, Direction.east) => (3, (0, y), Direction.east),
      (2, Direction.south) => (4, (0, last - x), Direction.east),
      (2, Direction.west) => (1, (last, y), Direction.west),
      (3, Direction.north) => (0, (x, last), Direction.north),
      (3, Direction.east) => (5, (last - y, 0), Direction.south),
      (3, Direction.south) => (4, (x, 0), Direction.south),
      (3, Direction.west) => (2, (last, y), Direction.west),
      (4, Direction.north) => (3, (x, last), Direction.north),
      (4, Direction.east) => (5, (0, y), Direction.east),
      (4, Direction.south) => (1, (last - x, last), Direction.north),
      (4, Direction.west) => (2, (last - y, last), Direction.north),
      (5, Direction.north) => (3, (last, last - x), Direction.west),
      (5, Direction.east) => (0, (last, last - y), Direction.west),
      (5, Direction.south) => (1, (0, last - x), Direction.east),
      (5, Direction.west) => (4, (last, y), Direction.west),
      _ => throw StateError('Invalid cube edge'),
    };
  }

  return switch ((face, direction)) {
    (0, Direction.north) => (5, (0, x), Direction.east),
    (0, Direction.east) => (1, (0, y), Direction.east),
    (0, Direction.south) => (2, (x, 0), Direction.south),
    (0, Direction.west) => (4, (0, last - y), Direction.east),
    (1, Direction.north) => (5, (x, last), Direction.north),
    (1, Direction.east) => (3, (last, last - y), Direction.west),
    (1, Direction.south) => (2, (last, x), Direction.west),
    (1, Direction.west) => (0, (last, y), Direction.west),
    (2, Direction.north) => (0, (x, last), Direction.north),
    (2, Direction.east) => (1, (y, last), Direction.north),
    (2, Direction.south) => (3, (x, 0), Direction.south),
    (2, Direction.west) => (4, (y, 0), Direction.south),
    (3, Direction.north) => (2, (x, last), Direction.north),
    (3, Direction.east) => (1, (last, last - y), Direction.west),
    (3, Direction.south) => (5, (last, x), Direction.west),
    (3, Direction.west) => (4, (last, y), Direction.west),
    (4, Direction.north) => (2, (0, x), Direction.east),
    (4, Direction.east) => (3, (0, y), Direction.east),
    (4, Direction.south) => (5, (x, 0), Direction.south),
    (4, Direction.west) => (0, (0, last - y), Direction.east),
    (5, Direction.north) => (4, (x, last), Direction.north),
    (5, Direction.east) => (3, (y, last), Direction.north),
    (5, Direction.south) => (1, (x, 0), Direction.south),
    (5, Direction.west) => (0, (y, 0), Direction.south),
    _ => throw StateError('Invalid cube edge'),
  };
}

(List<List<String>>, List<String>) _parse(List<String> input) {
  final rows = input.takeWhile((line) => line.isNotEmpty).toList();
  final width = rows.map((row) => row.length).max;
  final map = rows.map((row) => row.padRight(width)).grid();
  final path = RegExp(r'\d+|[LR]').allMatches(input.last).map((m) => m[0]!).toList();

  return (map, path);
}

List<Index> _offsets(bool sample) => sample
    ? [(2, 0), (0, 1), (1, 1), (2, 1), (2, 2), (3, 2)]
    : [(1, 0), (2, 0), (1, 1), (1, 2), (0, 2), (0, 3)];

List<List<List<String>>> _splitGrids(List<List<String>> map, int size, bool sample) =>
    _offsets(sample)
        .map(
          (offset) => map
              .skip(offset.y * size)
              .take(size)
              .map((row) => row.sublist(offset.x * size, (offset.x + 1) * size))
              .toList(),
        )
        .toList();
