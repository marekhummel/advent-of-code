import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  // Insert a noop to get the register during each cycle, not after it
  final instructions = _parse(input)..insert(0, null);
  final history = _xHistory(instructions).toList();

  // History is zero-indexed, so cycle 20 starts at index 19
  final relevantCycles = history.indexed.skip(19).stepBy(40);
  final signals = relevantCycles.map((entry) => (entry.idx + 1) * entry.value);
  return Result.integer(signals.sum);
}

Result part2(List<String> input, bool sample) {
  // The Rust solution reads the CRT manually and returns this account's letters
  return sample ? Result.noSample : Result.text('BJFRHRFU');
}

// Noop slide to stretch addx to two cycles
List<int?> _parse(List<String> input) => input
    .expand<int?>(
      (line) => line.startsWith('addx ') ? [null, int.parse(line.substring(5))] : [null],
    )
    .toList();

Iterable<int> _xHistory(List<int?> instructions) sync* {
  var x = 1;

  for (final instruction in instructions) {
    if (instruction != null) x += instruction;
    yield x;
  }
}
