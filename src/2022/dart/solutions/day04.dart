import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

typedef _Pair = (Range, Range);

Result part1(List<String> input, bool sample) =>
    Result.integer(_parse(input).where(_fullyContain).length);

Result part2(List<String> input, bool sample) =>
    Result.integer(_parse(input).where(_overlap).length);

List<_Pair> _parse(List<String> input) => input.map((line) {
  final values = line.split(RegExp('[-,]')).map(int.parse).toList();

  return (range(values[0], values[1]), range(values[2], values[3]));
}).toList();

bool _fullyContain(_Pair pair) {
  final (a, b) = pair;
  return a.containsRange(b) || b.containsRange(a);
}

bool _overlap(_Pair pair) {
  final (a, b) = pair;
  return a.overlaps(b);
}
