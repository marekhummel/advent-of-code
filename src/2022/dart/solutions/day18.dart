import 'package:collection/collection.dart';

import '../../../lib/dart/helpers.dart';
import '../../../lib/dart/runner.dart';

typedef _Cube = ({int x, int y, int z});

extension _CubeGeometry on _Cube {
  List<_Cube> get deltas => [
    (x: 1, y: 0, z: 0),
    (x: -1, y: 0, z: 0),
    (x: 0, y: 1, z: 0),
    (x: 0, y: -1, z: 0),
    (x: 0, y: 0, z: 1),
    (x: 0, y: 0, z: -1),
  ].map((delta) => (x: x + delta.x, y: y + delta.y, z: z + delta.z)).toList();
}

Result part1(List<String> input, bool sample) {
  final droplets = _parse(input);

  return Result.integer(_countSides(droplets, droplets.toSet()));
}

Result part2(List<String> input, bool sample) {
  final droplets = _parse(input);

  // Compute the bounds
  final xMin = droplets.map((c) => c.x).min;
  final xMax = droplets.map((c) => c.x).max;
  final yMin = droplets.map((c) => c.y).min;
  final yMax = droplets.map((c) => c.y).max;
  final zMin = droplets.map((c) => c.z).min;
  final zMax = droplets.map((c) => c.z).max;

  // Start with droplets as the cubes that cannot contain air
  final noAir = droplets.toSet();

  for (var x = xMin; x <= xMax; x++) {
    for (var y = yMin; y <= yMax; y++) {
      for (var z = zMin; z <= zMax; z++) {
        final center = (x: x, y: y, z: z);
        if (noAir.contains(center)) continue;

        // Flood fill from the current cube
        final pocket = <_Cube>{};
        final flood = breadthFirst(
          center,
          (cube) => cube.deltas.where((next) => !noAir.contains(next)),
        );
        var isInterior = true;
        for (final (cube, _) in flood) {
          pocket.add(cube);

          // Reaching outside the bounds means this is not an interior pocket
          if (!range(xMin, xMax).contains(cube.x) ||
              !range(yMin, yMax).contains(cube.y) ||
              !range(zMin, zMax).contains(cube.z)) {
            isInterior = false;
            break;
          }
        }

        // A fully enclosed pocket contributes no exposed sides
        if (isInterior) noAir.addAll(pocket);
      }
    }
  }

  return Result.integer(_countSides(droplets, noAir));
}

List<_Cube> _parse(List<String> input) => input.map((line) {
  final values = line.split(',').map(int.parse).toList();

  return (x: values[0], y: values[1], z: values[2]);
}).toList();

int _countSides(List<_Cube> droplets, Set<_Cube> noAir) => droplets
    .expand((droplet) => droplet.deltas)
    .where((neighbor) => !noAir.contains(neighbor))
    .length;
