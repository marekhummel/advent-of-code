import '../../../lib/dart/runner.dart';

typedef _Move = (int, int, int);

Result part1(List<String> input, bool sample) {
  final (stacks, moves) = _parse(input);

  for (final (count, from, to) in moves) {
    for (var i = 0; i < count; i++) {
      stacks[to - 1].add(stacks[from - 1].removeLast());
    }
  }

  return Result.text(stacks.map((stack) => stack.last).join());
}

Result part2(List<String> input, bool sample) {
  final (stacks, moves) = _parse(input);

  for (final (count, from, to) in moves) {
    final moving = <String>[];
    for (var i = 0; i < count; i++) {
      moving.add(stacks[from - 1].removeLast());
    }
    stacks[to - 1].addAll(moving.reversed);
  }

  return Result.text(stacks.map((stack) => stack.last).join());
}

(List<List<String>>, List<_Move>) _parse(List<String> input) {
  final separator = input.indexOf('');
  final initial = input.sublist(0, separator);
  final count = initial.last.replaceAll(RegExp(r'\s'), '').length;
  final stacks = List.generate(count, (_) => <String>[]);

  for (final line in initial.sublist(0, initial.length - 1).reversed) {
    for (var i = 0; i < line.length; i++) {
      if (RegExp('[A-Z]').hasMatch(line[i])) {
        stacks[(i - 1) ~/ 4].add(line[i]);
      }
    }
  }

  final moves = input.skip(separator + 1).map((line) {
    final words = line.split(' ');
    return (int.parse(words[1]), int.parse(words[3]), int.parse(words[5]));
  }).toList();

  return (stacks, moves);
}
