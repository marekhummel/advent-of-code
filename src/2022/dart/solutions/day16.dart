import 'dart:collection';
import 'dart:math';

import 'package:collection/collection.dart';

import '../../../lib/dart/runner.dart';

typedef _Tunnels = Map<int, List<(int, int)>>;

Result part1(List<String> input, bool sample) {
  final (flows, tunnels, valveAA) = _parse(input);
  final better = _updateTunnelNetwork(tunnels, flows, valveAA);

  return Result.integer(_computeBestFlows(better, flows, valveAA, 30).values.max);
}

Result part2(List<String> input, bool sample) {
  final (flows, tunnels, valveAA) = _parse(input);
  final better = _updateTunnelNetwork(tunnels, flows, valveAA);

  // Find the best flow for each set of opened valves
  final paths = _computeBestFlows(better, flows, valveAA, 26).entries.toList();

  // Combine two non-overlapping valve sets
  var optimal = 0;

  for (var i = 0; i < paths.length; i++) {
    for (var j = i + 1; j < paths.length; j++) {
      if (paths[i].key & paths[j].key == 0) {
        optimal = max(optimal, paths[i].value + paths[j].value);
      }
    }
  }

  return Result.integer(optimal);
}

(Map<int, int>, _Tunnels, int) _parse(List<String> input) {
  final pattern = RegExp(
    r'^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z]{2}(?:, )?)+)$',
  );

  final parsed = input.map((line) {
    final match = pattern.firstMatch(line)!;
    return (match[1]!, int.parse(match[2]!), match[3]!.split(', '));
  }).toList();

  // Assign compact valve IDs for graph indices and bit sets
  final valve_map = {for (var i = 0; i < parsed.length; i++) parsed[i].$1: i};
  final flow_map = {for (final (name, flow, _) in parsed) valve_map[name]!: flow};
  final tunnels = {
    for (final (name, _, neighbors) in parsed)
      valve_map[name]!: neighbors.map((name) => (valve_map[name]!, 1)).toList(),
  };

  return (flow_map, tunnels, valve_map['AA']!);
}

_Tunnels _updateTunnelNetwork(_Tunnels tunnels, Map<int, int> flows, int start_valve) {
  final size = tunnels.length;
  final distances = List.generate(size, (i) => List.generate(size, (j) => i == j ? 0 : 1000000));

  for (final entry in tunnels.entries) {
    for (final (neighbor, weight) in entry.value) {
      distances[entry.key][neighbor] = weight;
    }
  }

  // The same Floyd-Warshall reduction as Rust's Graph helper
  for (var k = 0; k < size; k++) {
    for (var i = 0; i < size; i++) {
      for (var j = 0; j < size; j++) {
        distances[i][j] = min(distances[i][j], distances[i][k] + distances[k][j]);
      }
    }
  }

  // Connect useful valves, retaining AA as the starting point
  final better = <int, List<(int, int)>>{};

  for (var a = 0; a < size; a++) {
    for (var b = a + 1; b < size; b++) {
      if ((flows[a] != 0 && (flows[b] != 0 || b == start_valve)) ||
          (flows[b] != 0 && (flows[a] != 0 || a == start_valve))) {
        better.putIfAbsent(a, () => []).add((b, distances[a][b]));
        better.putIfAbsent(b, () => []).add((a, distances[a][b]));
      }
    }
  }

  return better;
}

Map<int, int> _computeBestFlows(_Tunnels tunnels, Map<int, int> flows, int start, int maxTime) {
  // Total flow, time, current flow rate, location, open valves
  final queue = ListQueue<(int, int, int, int, int)>()..add((0, 0, 0, start, 0));
  final best = <int, int>{};

  while (queue.isNotEmpty) {
    final (total, time, rate, location, open) = queue.removeFirst();

    // Compute the final flow if we idle from this state
    final idleFlow = total + (maxTime - time) * rate;
    best[open] = max(best[open] ?? 0, idleFlow);
    if (time >= maxTime) continue;

    // Combine moving and opening to reduce the search tree
    for (final (target, weight) in tunnels[location] ?? <(int, int)>[]) {
      if ((open >> target) & 1 == 1) continue;

      // Compute the time needed to move and open the valve
      final distance = weight + 1;
      final nextTime = time + distance;
      if (nextTime > maxTime) continue;

      // Append the next state
      queue.add((
        total + distance * rate,
        nextTime,
        rate + flows[target]!,
        target,
        open | (1 << target),
      ));
    }
  }

  return best;
}
