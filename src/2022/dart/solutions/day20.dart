import 'package:collection/collection.dart';
import 'package:more/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

typedef _Links = (int, int);

Result part1(List<String> input, bool sample) {
  final numbers = input.map(int.parse).toList();
  final arrangement = _initialArrangement(numbers);
  _mix(numbers, arrangement);

  return Result.integer(_getCoordinates(numbers, arrangement));
}

Result part2(List<String> input, bool sample) {
  final numbers = input.map((line) => int.parse(line) * 811589153).toList();
  final arrangement = _initialArrangement(numbers);

  for (var i = 0; i < 10; i++) {
    _mix(numbers, arrangement);
  }

  return Result.integer(_getCoordinates(numbers, arrangement));
}

List<_Links> _initialArrangement(List<int> numbers) => List.generate(
  numbers.length,
  (i) => ((i + numbers.length - 1) % numbers.length, (i + 1) % numbers.length),
);

void _mix(List<int> numbers, List<_Links> arrangement) {
  for (var index = 0; index < numbers.length; index++) {
    // Compute the new index
    final steps = numbers[index] % (numbers.length - 1);
    var newIndex = index;
    for (var step = 0; step < steps; step++) {
      newIndex = arrangement[newIndex].$2;
    }

    if (index == newIndex) continue;

    // Remove the node at its old position
    final (previous, next) = arrangement[index];
    arrangement[previous] = (arrangement[previous].$1, next);
    arrangement[next] = (previous, arrangement[next].$2);

    // Insert the node at its new position
    final newNext = arrangement[newIndex].$2;
    arrangement[index] = (newIndex, newNext);
    arrangement[newIndex] = (arrangement[newIndex].$1, index);
    arrangement[newNext] = (index, arrangement[newNext].$2);
  }
}

int _getCoordinates(List<int> numbers, List<_Links> arrangement) {
  final chain = iterate(numbers.indexOf(0), (index) => arrangement[index].$2);

  // Read the 1000th, 2000th, and 3000th values after zero
  return chain.stepBy(1000).skip(1).take(3).map((index) => numbers[index]).sum;
}
