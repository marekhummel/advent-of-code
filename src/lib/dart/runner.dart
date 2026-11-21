import 'dart:io';

class Result {
  final Object value;

  const Result.number(num value) : value = value;
  const Result.text(String value) : value = value;

  static const noSample = Result.text('<No Sample Defined>');
  static const noPartTwo = Result.text('<No Part Two>');
  static const notImplemented = Result.text('<Not Implemented>');
  static const unsolved = Result.text('<Unsolved>');

  @override
  String toString() => value.toString();
}

typedef Part = Result Function(List<String> input, bool sample);

class Solution {
  final Part? part1;
  final Part? part2;

  const Solution({this.part1, this.part2});
}

/// Run one DAY PART MODE case; scripts handle batches and verification.
void run(int year, Map<int, Solution> solutions, List<String> args) {
  final day = args.isNotEmpty ? int.tryParse(args[0]) : null;
  final part = args.length > 1 ? int.tryParse(args[1]) : null;
  final mode = args.length > 2 ? args[2] : null;
  final lastDay = year < 2025 ? 25 : 12;
  if (args.length != 3 ||
      day == null ||
      day < 1 ||
      day > lastDay ||
      (part != 1 && part != 2) ||
      (mode != 'real' && mode != 'sample')) {
    stderr.writeln('Usage: main.dart DAY PART MODE (real|sample)');
    exit(2);
  }

  final solution = solutions[day];
  if (solution == null) {
    stdout.writeln(Result.unsolved);
    return;
  }
  final solve = part == 1 ? solution.part1 : solution.part2;
  if (solve == null) {
    stdout.writeln(Result.notImplemented);
    return;
  }

  final sample = mode == 'sample';
  final prefix = sample ? 'sample' : 'input';
  final stem = 'inputs/$year/$prefix${day.toString().padLeft(2, '0')}';
  List<String> input;
  try {
    try {
      input = File('$stem.txt').readAsLinesSync();
    } on FileSystemException catch (error) {
      // Only a missing file should select the part-specific fallback.
      if (error.osError?.errorCode != 2) rethrow;
      input = File('${stem}_$part.txt').readAsLinesSync();
    }
  } on FileSystemException catch (error) {
    stderr.writeln('Error: Cannot read input: $error');
    exit(1);
  }

  stdout.writeln(solve(input, sample));
}
