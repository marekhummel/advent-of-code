import 'package:more/math.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/input.dart';
import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final monkeys = _parse(input);

  for (var i = 0; i < 20; i++) {
    _round(monkeys, (worry) => worry ~/ 3);
  }

  final inspections = monkeys.map((monkey) => monkey.inspectedItems).sortedDesc().take(2);
  return Result.integer(inspections.product);
}

Result part2(List<String> input, bool sample) {
  final monkeys = _parse(input);

  // Limit worry to 0..lcm(tests), without affecting divisibility tests
  final commonMultiple = monkeys.map((monkey) => monkey.test).lcm();

  for (var i = 0; i < 10000; i++) {
    _round(monkeys, (worry) => worry % commonMultiple);
  }

  final inspections = monkeys.map((monkey) => monkey.inspectedItems).sortedDesc().take(2);
  return Result.integer(inspections.product);
}

class _Monkey {
  final List<int> items;
  final (String, String) operation;
  final int test;
  final int ifTrue;
  final int ifFalse;
  var inspectedItems = 0;

  _Monkey(this.items, this.operation, this.test, this.ifTrue, this.ifFalse);

  factory _Monkey.fromLines(List<String> lines) {
    final operation = lines[2].substring('  Operation: new = old '.length).split(' ');
    return _Monkey(
      lines[1]
          .substring('  Starting items: '.length)
          .split(',')
          .map((s) => int.parse(s.trim()))
          .toList(),
      (operation[0], operation[1]),
      int.parse(lines[3].substring('  Test: divisible by '.length)),
      int.parse(lines[4].substring('    If true: throw to monkey '.length)),
      int.parse(lines[5].substring('    If false: throw to monkey '.length)),
    );
  }
}

List<_Monkey> _parse(List<String> input) => input.blocks().map(_Monkey.fromLines).toList();

void _round(List<_Monkey> monkeys, int Function(int) manageWorry) {
  for (final monkey in monkeys) {
    for (final item in monkey.items) {
      final (op, value_str) = monkey.operation;
      final value = value_str == 'old' ? item : int.parse(value_str);
      final worry = manageWorry(switch (op) {
        '+' => item + value,
        '*' => item * value,
        _ => throw StateError('Invalid operation'),
      });
      final next = worry % monkey.test == 0 ? monkey.ifTrue : monkey.ifFalse;
      monkeys[next].items.add(worry);
    }
    monkey.inspectedItems += monkey.items.length;
    monkey.items.clear();
  }
}
