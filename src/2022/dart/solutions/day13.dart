import 'dart:collection';

import 'package:collection/collection.dart';
import 'package:more/comparator.dart';

import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final packets = _parse(input);
  final indices = packets
      .slices(2)
      .where((pair) => pair.length == 2)
      .mapIndexed((index, pair) => pair[0].compareTo(pair[1]) <= 0 ? index + 1 : 0);

  return Result.integer(indices.sum);
}

Result part2(List<String> input, bool sample) {
  final packets = _parse(input);

  // Add the divider packets before sorting
  final divider1 = _PacketList([
    _PacketList([_PacketInt(2)]),
  ]);
  final divider2 = _PacketList([
    _PacketList([_PacketInt(6)]),
  ]);
  packets.addAll([divider1, divider2]);

  // Sort by the recursive packet ordering
  packets.sort();

  // Find the one-based divider positions
  final distress1 = packets.indexWhere((packet) => packet.compareTo(divider1) == 0) + 1;
  final distress2 = packets.indexWhere((packet) => packet.compareTo(divider2) == 0) + 1;

  return Result.integer(distress1 * distress2);
}

// Sealed classes represent Rust's Packet enum without dynamic JSON values
sealed class _Packet implements Comparable<_Packet> {
  static _Packet parse(ListQueue<String> tokens) {
    final next = tokens.removeFirst();
    final number = int.tryParse(next);
    if (number != null) return _PacketInt(number);

    assert(next == '[');
    if (tokens.first == ']') {
      tokens.removeFirst();
      return _PacketList([]);
    }

    final children = <_Packet>[];
    do {
      children.add(parse(tokens));
    } while (tokens.removeFirst() != ']');
    return _PacketList(children);
  }

  @override
  int compareTo(_Packet other) {
    if (this case _PacketInt(:final value)) {
      if (other case _PacketInt(value: final right)) return value.compareTo(right);
      return _PacketList([this]).compareTo(other);
    }

    if (other is _PacketInt) return -other.compareTo(this);
    final left = (this as _PacketList).children;
    final right = (other as _PacketList).children;
    final compare = (_Packet a, _Packet b) => a.compareTo(b);
    return compare.lexicographical(left, right);
  }
}

final class _PacketInt extends _Packet {
  final int value;
  _PacketInt(this.value);
}

final class _PacketList extends _Packet {
  final List<_Packet> children;
  _PacketList(this.children);
}

List<_Packet> _parse(List<String> input) {
  final tokenPattern = RegExp(r'(\[|\]|,|\d+)');

  return input.where((line) => line.isNotEmpty).map((line) {
    final tokens = ListQueue<String>.from(tokenPattern.allMatches(line).map((match) => match[0]!));
    return _Packet.parse(tokens);
  }).toList();
}
