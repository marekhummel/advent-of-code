import 'dart:collection';
import 'dart:math';

extension IterableHelpers<T> on Iterable<T> {
  List<T> sortedAsc() => toList()..sort();

  List<T> sortedDesc() => sortedAsc().reversed.toList();

  /// Yield the first element and then every step-th element, lazily
  Iterable<T> stepBy(int step) sync* {
    if (step <= 0) throw RangeError.value(step, 'step', 'Must be positive');
    final iterator = this.iterator;
    while (iterator.moveNext()) {
      yield iterator.current;
      for (var skipped = 1; skipped < step; skipped++) {
        if (!iterator.moveNext()) return;
      }
    }
  }
}

/// Named access to entries produced by Iterable.indexed
extension IndexedEntry<T> on (int, T) {
  int get idx => $1;
  T get value => $2;
}

extension NumberProduct on Iterable<num> {
  /// Multiply all values, returning 1 for an empty iterable
  num get product => fold<num>(1, (product, value) => product * value);
}

extension IntegerProduct on Iterable<int> {
  /// Integer specialization preserves the result type, like collection's sum
  int get product => fold(1, (product, value) => product * value);
}

/// An (x, y) coordinate, also usable outside a grid or with negative values
typedef Index = (int, int);

/// Compass order is north, east, south, west; y grows down
enum Direction {
  north((0, -1)),
  east((1, 0)),
  south((0, 1)),
  west((-1, 0));

  final Index offset;
  const Direction(this.offset);

  Direction get left => values[(this.index + 3) % 4];
  Direction get right => values[(this.index + 1) % 4];
  Direction get opposite => values[(this.index + 2) % 4];
}

/// Eight surrounding offsets in order: N NW E NE S SE W SW
const List<Index> _mooreOffsets = [
  (0, -1),
  (-1, -1),
  (1, 0),
  (1, -1),
  (0, 1),
  (1, 1),
  (-1, 0),
  (-1, 1),
];

extension IndexGeometry on Index {
  int get x => $1;
  int get y => $2;

  Index operator +(Index offset) => (x + offset.x, y + offset.y);

  int manhattanDistance(Index other) => (x - other.x).abs() + (y - other.y).abs();

  /// Whether this coordinate is inside a rectangle starting at (0, 0)
  bool isInside(int width, int height) => x >= 0 && x < width && y >= 0 && y < height;

  /// Unbounded neighbors; filter with [isInside] when working on a grid
  Iterable<Index> get vonNeumannNeighbors =>
      Direction.values.map((direction) => this + direction.offset);

  Iterable<Index> get mooreNeighbors => _mooreOffsets.map((offset) => this + offset);
}

/// Grid helpers keep storage as ordinary rows and support uneven row lengths
extension Grid<T> on List<List<T>> {
  Iterable<Index> get indices sync* {
    for (var y = 0; y < length; y++) {
      for (var x = 0; x < this[y].length; x++) {
        yield (x, y);
      }
    }
  }

  /// Create filled rows
  static List<List<E>> create<E>(int width, int height, E fill) =>
      List.generate(height, (_) => List.filled(width, fill));

  bool containsIndex(Index index) =>
      index.y >= 0 && index.y < length && index.x >= 0 && index.x < this[index.y].length;

  T at(Index index) => this[index.y][index.x];

  void setAt(Index index, T value) => this[index.y][index.x] = value;

  List<List<R>> mapCells<R>(R Function(T) convert) =>
      map((row) => row.map(convert).toList()).toList();
}

/// An inclusive range (start, end), with start <= end
typedef Range = ({int start, int end});

Range range(int start, int end) => (start: start, end: end);

extension RangeOperations on Range {
  int get length => end - start + 1;

  bool contains(int value) => start <= value && value <= end;

  bool containsRange(Range other) => start <= other.start && other.end <= end;

  bool overlaps(Range other) => start <= other.end && other.start <= end;

  /// The smallest range covering both, including any gap between them
  Range span(Range other) => range(min(start, other.start), max(end, other.end));
}

/// Visit each reachable node once, paired with its shortest unit-edge distance
/// Stop iterating to end the search early, before expanding the current node
Iterable<(T, int)> breadthFirst<T>(T start, Iterable<T> Function(T) neighbors) sync* {
  final seen = <T>{start};
  final queue = ListQueue<(T, int)>()..add((start, 0));
  while (queue.isNotEmpty) {
    final (node, distance) = queue.removeFirst();
    yield (node, distance);

    for (final next in neighbors(node)) {
      if (seen.add(next)) queue.add((next, distance + 1));
    }
  }
}
