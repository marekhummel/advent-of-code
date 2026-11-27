import 'dart:math';

import 'package:collection/collection.dart';
import 'package:more/comparator.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

typedef _Blueprint = List<List<int>>;
typedef _State = (int, List<int>, List<int>, int);
typedef _Robots = (int, int, int, int);

Result part1(List<String> input, bool sample) {
  final blueprints = _parse(input);
  final qualities = blueprints.mapIndexed((i, bp) => (i + 1) * _emulate(bp, 24));

  return Result.integer(qualities.sum);
}

Result part2(List<String> input, bool sample) {
  final blueprints = _parse(input);
  final most_geodes = blueprints.take(3).map((bp) => _emulate(bp, 32));

  final product = most_geodes.product;
  return Result.integer(product);
}

List<_Blueprint> _parse(List<String> input) {
  const mineralIndices = {'ore': 0, 'clay': 1, 'obsidian': 2};

  return input.map((line) {
    final text = line.substring(line.indexOf(': ') + 2);
    final robots = text.split('.').where((robot) => robot.contains('costs '));
    final blueprint = Grid.create(4, 4, 0);
    var robot = 0;
    for (final text in robots) {
      final costs = text.substring(text.indexOf('costs ') + 6).split(' and ');
      for (final cost in costs) {
        final parts = cost.split(' ');
        blueprint[robot][mineralIndices[parts[1]]!] += int.parse(parts[0]);
      }
      robot++;
    }
    return blueprint;
  }).toList();
}

/// Special function to allow robots as keys in sets
_Robots _robotKey(List<int> robots) => (robots[0], robots[1], robots[2], robots[3]);

/// Heap sort function
int _compareStates(_State a, _State b) {
  final compareList = ((int a, int b) => a.compareTo(b)).lexicographical;

  var comparison = b.$1.compareTo(a.$1);
  if (comparison != 0) return comparison;

  comparison = compareList(b.$2, a.$2);
  if (comparison != 0) return comparison;

  comparison = compareList(b.$3, a.$3);
  if (comparison != 0) return comparison;

  return b.$4.compareTo(a.$4);
}

int _emulate(_Blueprint blueprint, int maxTime) {
  final maxNeeded = List.generate(3, (m) => blueprint.map((robot) => robot[m]).max);
  final queue = HeapPriorityQueue<_State>(_compareStates)..add((0, [1, 0, 0, 0], [0, 0, 0, 0], 0));
  final seen = <_Robots, int>{};
  var mostGeodes = 0;

  while (queue.isNotEmpty) {
    final (_, robots, minerals, time) = queue.removeFirst();

    // Last iteration
    if (time == maxTime) {
      mostGeodes = max(mostGeodes, minerals[3]);
      continue;
    }
    seen[_robotKey(robots)] = time;

    // Try to build each robot
    var canBuildAll = true;
    for (var r = 0; r < 4; r++) {
      // Check whether the resources are available
      if (!blueprint[r].indexed.every((cost) => cost.value <= minerals[cost.idx])) {
        canBuildAll = false;
        continue;
      }

      // Do not build more robots than needed
      if (r != 3 && robots[r] >= maxNeeded[r]) continue;

      final newRobots = List<int>.of(robots);
      newRobots[r]++;

      // Prune robot counts already reached at an earlier time
      final previousTime = seen[_robotKey(newRobots)];
      if (previousTime != null && previousTime <= time + 1) continue;

      // Mine and build the new robot if the bound can improve our best result
      final newMinerals = List.generate(4, (i) => minerals[i] + robots[i] - blueprint[r][i]);
      final maxPossible = _maxPossibleGeodes(newRobots, newMinerals, maxTime - time);
      if (maxPossible > mostGeodes) {
        queue.add((maxPossible, newRobots, newMinerals, time + 1));
      }
    }

    // If not all robots are affordable, it may be worth saving resources
    if (!canBuildAll) {
      // Only proceed if waiting can still improve our best result
      final newMinerals = List.generate(4, (i) => minerals[i] + robots[i]);
      final maxPossible = _maxPossibleGeodes(robots, newMinerals, maxTime - time);
      if (maxPossible > mostGeodes) queue.add((maxPossible, robots, newMinerals, time + 1));
    }
  }

  return mostGeodes;
}

/// Maximum possible collected geodes, based on current state.
/// Assumes that every minute a new geode robot is build and farms.
int _maxPossibleGeodes(List<int> robots, List<int> minerals, int remaining) =>
    minerals[3] + remaining * robots[3] + remaining * (remaining + 1) ~/ 2;
