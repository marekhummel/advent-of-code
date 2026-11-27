import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

typedef _Rule = (List<int>, Direction);

Result part1(List<String> input, bool sample) {
  final elves = _parse(input);
  final rules = _proposalRules();

  for (var i = 0; i < 10; i++) {
    _round(elves, rules);
  }

  final xMin = elves.map((p) => p.x).min;
  final xMax = elves.map((p) => p.x).max;
  final yMin = elves.map((p) => p.y).min;
  final yMax = elves.map((p) => p.y).max;

  return Result.integer((xMax - xMin + 1) * (yMax - yMin + 1) - elves.length);
}

Result part2(List<String> input, bool sample) {
  final elves = _parse(input);
  final rules = _proposalRules();
  var round = 1;

  while (_round(elves, rules)) {
    round++;
  }

  return Result.integer(round);
}

Set<Index> _parse(List<String> input) => {
  for (var y = 0; y < input.length; y++)
    for (var x = 0; x < input[y].length; x++)
      if (input[y][x] == '#') (x, y),
};

bool _round(Set<Index> elves, List<_Rule> rules) {
  // Collect all proposals before moving any elf
  final proposals = {for (final elf in elves) elf: _proposal(elf, elves, rules)};
  if (proposals.values.every((p) => p.$2)) return false;

  // Count conflicts at each target position
  final counts = proposals.values.groupFoldBy<Index, int>(
    (proposal) => proposal.$1,
    (count, _) => (count ?? 0) + 1,
  );

  // Only apply proposals with a unique target
  for (final entry in proposals.entries) {
    final (target, idle) = entry.value;
    if (!idle && counts[target] == 1) {
      elves.remove(entry.key);
      elves.add(target);
    }
  }

  // Rotate the direction priorities for the next round
  rules.add(rules.removeAt(0));

  return true;
}

(Index, bool) _proposal(Index position, Set<Index> elves, List<_Rule> rules) {
  final neighbors = position.mooreNeighbors.toList();
  if (neighbors.every((neighbor) => !elves.contains(neighbor))) return (position, true);

  for (final (indices, direction) in rules) {
    if (indices.every((i) => !elves.contains(neighbors[i]))) {
      return (position + direction.offset, false);
    }
  }

  return (position, false);
}

List<_Rule> _proposalRules() => [
  ([0, 1, 3], Direction.north),
  ([4, 5, 7], Direction.south),
  ([1, 6, 7], Direction.west),
  ([2, 3, 5], Direction.east),
];
