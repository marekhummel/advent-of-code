import 'package:collection/collection.dart';

import '../../../lib/dart/runner.dart';

Result part1(List<String> input, bool sample) {
  final tree = _parse(input);

  final sizes = tree.keys.map((dir) => _dirSize(dir, tree));

  return Result.integer(sizes.where((size) => size <= 100000).sum);
}

Result part2(List<String> input, bool sample) {
  final tree = _parse(input);

  // Compute the space that must be freed
  final occupied = _dirSize('', tree);
  final unusedSpace = 70000000 - occupied;
  final toBeDeletedSpace = 30000000 - unusedSpace;

  final sizes = tree.keys.map((dir) => _dirSize(dir, tree));

  return Result.integer(sizes.where((size) => size > toBeDeletedSpace).min);
}

sealed class _Entry {}

final class _File extends _Entry {
  final String name;
  final int size;
  _File(this.name, this.size);
}

final class _Directory extends _Entry {
  final String name;
  _Directory(this.name);
}

Map<String, List<_Entry>> _parse(List<String> input) {
  // Dart lists use identity equality; slash-separated paths for keys
  final tree = <String, List<_Entry>>{};
  final cwd = <String>[];

  for (final line in input) {
    if (line.startsWith(r'$ ')) {
      final command = line.substring(2);

      // CD
      if (command.startsWith('cd ')) {
        final dir = command.substring(3);
        switch (dir) {
          case '/':
            cwd.clear();
          case '..':
            cwd.removeLast();
          default:
            cwd.add(dir);
        }
      }
    } else {
      // Output line of LS command
      final _Entry entry;
      if (line.startsWith('dir ')) {
        entry = _Directory(line.substring(4));
      } else {
        final separator = line.indexOf(' ');
        entry = _File(line.substring(separator + 1), int.parse(line.substring(0, separator)));
      }
      tree.putIfAbsent(cwd.join('/'), () => []).add(entry);
    }
  }

  return tree;
}

int _dirSize(String directory, Map<String, List<_Entry>> files) => files[directory]!
    .map(
      (entry) => switch (entry) {
        _File(:final size) => size,
        _Directory(:final name) => _dirSize(directory.isEmpty ? name : '$directory/$name', files),
      },
    )
    .sum;
