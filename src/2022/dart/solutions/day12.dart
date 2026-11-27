import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/input.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final grid = input.grid();
  final goal = _find(grid, 'E');
  final start = _find(grid, 'S');

  return Result.integer(_distances(grid, goal)[start]!);
}

Result part2(List<String> input, bool sample) {
  final grid = input.grid();
  final distances = _distances(grid, _find(grid, 'E'));
  final trails = grid.indices
      .where((index) => grid.at(index) == 'S' || grid.at(index) == 'a')
      .map((index) => distances[index])
      .nonNulls;

  return Result.integer(trails.min);
}

Index _find(List<List<String>> grid, String char) =>
    grid.indices.firstWhere((index) => grid.at(index) == char);

int _height(String char) => switch (char) {
  'S' => 0,
  'E' => 25,
  _ => char.codeUnitAt(0) - 97,
};

Map<Index, int> _distances(List<List<String>> grid, Index goal) {
  // Build the graph from the peak back towards the possible starts
  final paths = breadthFirst(
    goal,
    (current) => current.vonNeumannNeighbors
        .where(grid.containsIndex)
        .where((next) => _height(grid.at(current)) <= _height(grid.at(next)) + 1),
  );

  return {for (final (index, distance) in paths) index: distance};
}
