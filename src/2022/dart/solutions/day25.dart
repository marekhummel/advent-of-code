import 'package:collection/collection.dart';

import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final lookup = {for (var i = 0; i < 5; i++) '=-012'[i]: i - 2};

  var totalFuel = input
      .map((snafu) => snafu.split('').fold(0, (number, digit) => number * 5 + lookup[digit]!))
      .sum;

  // Convert the total fuel requirement back to SNAFU
  final reverseLookup = {for (final entry in lookup.entries) entry.value: entry.key};
  final console = <String>[];
  var carry = false;

  while (totalFuel > 0) {
    final digit = totalFuel % 5 + (carry ? 1 : 0);
    carry = digit > 2;
    totalFuel ~/= 5;
    console.add(reverseLookup[carry ? digit - 5 : digit]!);
  }

  // Append any remaining carry
  if (carry) console.add('1');

  return Result.text(console.reversed.join());
}

Result part2(List<String> input, bool sample) => Result.noPartTwo;
