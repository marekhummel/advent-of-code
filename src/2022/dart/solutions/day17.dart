import 'dart:math';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) =>
    Result.integer(-_dropAllRocks(2022, _parse(input)));

Result part2(List<String> input, bool sample) =>
    Result.integer(-_dropAllRocks(1000000000000, _parse(input)));

List<Direction> _parse(List<String> input) => input
    .join('\n')
    .split('')
    .map(
      (char) => switch (char) {
        '<' => Direction.west,
        '>' => Direction.east,
        _ => throw StateError('Invalid jet'),
      },
    )
    .toList();

const _rocks = [
  [(0, 0), (1, 0), (2, 0), (3, 0)],
  [(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)],
  [(0, 0), (1, 0), (2, 0), (2, -1), (2, -2)],
  [(0, 0), (0, -1), (0, -2), (0, -3)],
  [(0, 0), (1, 0), (0, -1), (1, -1)],
];

int _dropAllRocks(int totalRocks, List<Direction> jets) {
  final chamber = <Index>{};
  var top = 0;
  var jetIndex = 0;
  final seen = <(int, int), (int, int)>{};

  for (var step = 0; step < totalRocks; step++) {
    final i = step % _rocks.length;
    final rock = _rocks[i];
    final key = (i, jetIndex);

    // Caching
    final previous = seen[key];
    if (previous != null) {
      final (previousStep, previousTop) = previous;
      final cycle = step - previousStep;
      final cycles = (totalRocks - step) ~/ cycle;
      final remaining = (totalRocks - step) % cycle;

      // Preserve Rust's cycle key and exact-divisibility extrapolation
      if (remaining == 0) return top + (top - previousTop) * cycles;
    }

    seen[key] = (step, top);

    // Let the next rock fall
    var position = (2, top - 4);
    while (true) {
      // Jet
      final jet = jets[jetIndex];
      jetIndex = (jetIndex + 1) % jets.length;

      // Check whether the jet can push the rock sideways
      final pushed = rock.map((block) => position + block + jet.offset);
      if (pushed.every((next) => !chamber.contains(next) && range(0, 6).contains(next.x))) {
        position += jet.offset;
      }

      // Drop
      final fallen = rock.map((block) => position + block + Direction.south.offset);
      if (fallen.every((next) => !chamber.contains(next) && next.y < 0)) {
        position += Direction.south.offset;
      } else {
        // Rock rests
        for (final block in rock) {
          final resting = position + block;
          chamber.add(resting);
          top = min(top, resting.y);
        }
        break;
      }
    }
  }

  return top;
}
