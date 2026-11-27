import 'dart:collection';

import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  return Result.integer(_getRoot(ListQueue.of(_parse(input)), false).eval()!);
}

Result part2(List<String> input, bool sample) {
  final root = _getRoot(ListQueue.of(_parse(input)), true) as _Op;

  if (root.a.eval() != null) throw StateError('Expected unknown on the left');
  return Result.integer(root.a.solve(root.b.eval()!));
}

// Type definitions

enum _Operation { add, sub, mul, div }

sealed class _Yell {}

final class _YellNumber extends _Yell {
  final int number;
  _YellNumber(this.number);
}

final class _YellOperation extends _Yell {
  final String a;
  final String b;
  final _Operation operation;
  _YellOperation(this.a, this.b, this.operation);
}

sealed class _Term {
  int? eval() {
    switch (this) {
      case _Op(:final a, :final b, :final operation):
        final left = a.eval();
        final right = b.eval();
        if (left == null || right == null) return null;
        return switch (operation) {
          _Operation.add => left + right,
          _Operation.sub => left - right,
          _Operation.mul => left * right,
          _Operation.div => left ~/ right,
        };
      case _Number(:final number):
        return number;
      case _Unknown():
        return null;
    }
  }

  int solve(int target) {
    switch (this) {
      case _Unknown():
        return target;
      case _Number():
        throw StateError('No unknown to solve');
      case _Op(:final a, :final b, :final operation):
        final left = a.eval();
        final right = b.eval();
        if (left == null && right != null) {
          return a.solve(switch (operation) {
            _Operation.add => target - right,
            _Operation.sub => target + right,
            _Operation.mul => target ~/ right,
            _Operation.div => target * right,
          });
        }

        if (left != null && right == null) {
          return b.solve(switch (operation) {
            _Operation.add => target - left,
            _Operation.sub => -(target - left),
            _Operation.mul => target ~/ left,
            _Operation.div => 1 ~/ (target ~/ left),
          });
        }
        throw StateError('Expected exactly one unknown operand');
    }
  }
}

final class _Op extends _Term {
  final _Term a;
  final _Term b;
  final _Operation operation;
  _Op(this.a, this.b, this.operation);
}

final class _Number extends _Term {
  final int number;
  _Number(this.number);
}

final class _Unknown extends _Term {}

// Helpers

List<(String, _Yell)> _parse(List<String> input) => input.map((line) {
  final separator = line.indexOf(': ');
  final name = line.substring(0, separator);
  final message = line.substring(separator + 2);
  final number = int.tryParse(message);

  if (number != null) return (name, _YellNumber(number) as _Yell);

  final parts = message.split(' ');
  final operation = switch (parts[1]) {
    '+' => _Operation.add,
    '-' => _Operation.sub,
    '*' => _Operation.mul,
    '/' => _Operation.div,
    _ => throw StateError('Invalid operation'),
  };

  return (name, _YellOperation(parts[0], parts[2], operation) as _Yell);
}).toList();

_Term _getRoot(ListQueue<(String, _Yell)> monkeys, bool withHuman) {
  // Build expressions as soon as both operands are available
  final yelled = <String, _Term>{};

  while (monkeys.isNotEmpty) {
    final (name, yell) = monkeys.removeFirst();
    switch (yell) {
      case _YellNumber(:final number):
        yelled[name] = withHuman && name == 'humn' ? _Unknown() : _Number(number);
      case _YellOperation(:final a, :final b, :final operation):
        final left = yelled[a];
        final right = yelled[b];

        // Both operands are available, so this expression can be built
        if (left != null && right != null) {
          final result = _Op(left, right, operation);
          if (name == 'root') return result;
          yelled[name] = result;
        } else {
          monkeys.add((name, yell));
        }
    }
  }

  throw StateError('No root expression');
}
