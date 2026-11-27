import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/input.dart';
import '../../../lib/dart/runner.dart';

typedef _State = (Index, int);
typedef _Valley = List<List<String>>;

Result part1(List<String> input, bool sample) {
  final (valley, start, goal) = _parse(input);

  return Result.integer(_fastest(_Graph(valley), start, goal, 0));
}

Result part2(List<String> input, bool sample) {
  final (valley, start, goal) = _parse(input);
  final graph = _Graph(valley);

  // Travel to the goal, back to the start, and to the goal again
  var totalTime = _fastest(graph, start, goal, 0);
  totalTime += _fastest(graph, goal, start, totalTime);
  totalTime += _fastest(graph, start, goal, totalTime);

  return Result.integer(totalTime);
}

(_Valley, Index, Index) _parse(List<String> input) {
  // A tile is '.', '#', or a string of arrows, replacing Rust's tile enum
  final valley = input.grid();

  return (valley, (valley.first.indexOf('.'), 0), (valley.last.indexOf('.'), valley.length - 1));
}

class _Graph {
  final List<_Valley> history;
  _Graph(_Valley valley) : history = [valley];

  List<_State> adjacent(_State state) {
    final (index, time) = state;

    // Update the blizzards for the next minute
    while (history.length <= time + 1) {
      history.add(_moveBlizzards(history.last));
    }

    final valley = history[time + 1];

    // Find the possible next moves, including waiting
    return [...index.vonNeumannNeighbors, index]
        .where((next) => valley.containsIndex(next) && valley.at(next) == '.')
        .map((next) => (next, time + 1))
        .toList();
  }
}

int _fastest(_Graph graph, Index start, Index goal, int time) {
  // Local A*
  final queue = HeapPriorityQueue<(int, int, _State)>((a, b) => a.$1.compareTo(b.$1))
    ..add((start.manhattanDistance(goal), 0, (start, time)));

  final best = <_State, int>{(start, time): 0};

  while (queue.isNotEmpty) {
    final (_, cost, state) = queue.removeFirst();
    if (cost != best[state]) continue;
    if (state.$1 == goal) return cost;

    for (final next in graph.adjacent(state)) {
      final nextCost = cost + 1;
      if (best.containsKey(next) && best[next]! <= nextCost) continue;
      best[next] = nextCost;
      queue.add((nextCost + next.$1.manhattanDistance(goal), nextCost, next));
    }
  }

  throw StateError('No route through valley');
}

_Valley _moveBlizzards(_Valley current) {
  final height = current.length;
  final width = current[0].length;
  final next = Grid.create(width, height, '.');

  for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
      final tile = current[y][x];

      if (tile == '.') continue;
      if (tile == '#') {
        next[y][x] = '#';
        continue;
      }

      for (final direction in tile.split('')) {
        final heading = switch (direction) {
          '^' => Direction.north,
          '>' => Direction.east,
          'v' => Direction.south,
          '<' => Direction.west,
          _ => throw StateError('Invalid blizzard'),
        };
        final (dx, dy) = heading.offset;
        var nx = x + dx;
        var ny = y + dy;

        // Wrap blizzards around the interior when they reach a wall
        if (current[ny][nx] == '#') {
          nx = (nx + dx) % width + dx;
          ny = (ny + dy) % height + dy;
        }
        final position = (nx, ny);
        final previous = next.at(position);
        next.setAt(position, previous == '.' ? direction : previous + direction);
      }
    }
  }

  return next;
}
