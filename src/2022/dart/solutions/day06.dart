import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) => Result.integer(_findMarker(input.join('\n'), 4));

Result part2(List<String> input, bool sample) => Result.integer(_findMarker(input.join('\n'), 14));

int _findMarker(String datastream, int length) {
  for (var i = 0; i + length <= datastream.length; i++) {
    if (datastream.substring(i, i + length).split('').toSet().length == length) {
      return i + length;
    }
  }

  throw StateError('No marker found');
}
