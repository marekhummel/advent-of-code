import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) => Result.integer(_moveRope(_parse(input), 2));

Result part2(List<String> input, bool sample) => Result.integer(_moveRope(_parse(input), 10));

List<(Direction, int)> _parse(List<String> input) => input.map((line) {
  final parts = line.split(' ');
  final direction = switch (parts[0]) {
    'U' => Direction.north,
    'D' => Direction.south,
    'L' => Direction.west,
    'R' => Direction.east,
    _ => throw StateError('Invalid direction'),
  };

  return (direction, int.parse(parts[1]));
}).toList();

int _moveRope(List<(Direction, int)> motions, int ropeSize) {
  final rope = List<Index>.filled(ropeSize, (0, 0));

  final visited = {rope.last};

  for (final (dir, steps) in motions) {
    for (var step = 0; step < steps; step++) {
      rope[0] = rope[0] + dir.offset;
      for (var i = 1; i < ropeSize; i++) {
        final dx = rope[i - 1].x - rope[i].x;
        final dy = rope[i - 1].y - rope[i].y;

        // Move one step towards the previous knot when they no longer touch
        if (dx.abs() > 1 || dy.abs() > 1) {
          rope[i] = rope[i] + (dx.sign, dy.sign);
        }
      }
      visited.add(rope.last);
    }
  }

  return visited.length;
}
