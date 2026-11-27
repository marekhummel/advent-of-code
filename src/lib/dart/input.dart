extension Input on Iterable<String> {
  /// Nonempty blocks separated by empty lines; separators are omitted
  Iterable<List<String>> blocks() sync* {
    var block = <String>[];
    for (final line in this) {
      if (line.isEmpty) {
        if (block.isNotEmpty) yield block;
        block = [];
      } else {
        block.add(line);
      }
    }
    if (block.isNotEmpty) yield block;
  }

  /// One string per character. Use mapCells to parse a grid's values
  List<List<String>> grid() => map((line) => line.split('')).toList();
}
