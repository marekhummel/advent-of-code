from lib.python import ProblemInput, ProblemResult, Solution

# Observation: All shapes are 3x3, almost fully filled.
# Thus, optimal packing is just filling as many full 3x3 blocks as possible,
# there is no room anyways for clever fitting.

SHAPE_SIZE = 3


class Solution12(Solution):
    @staticmethod
    def results() -> list[ProblemResult]:
        return [3, 448, None, None]

    @staticmethod
    def solve_part01(data: ProblemInput, is_sample: bool) -> ProblemResult:
        shape_defs, regions = parse(data)
        shape_sizes = [len(defn) for defn in shape_defs]

        valid, invalid, unsure = 0, 0, 0
        for (width, height), shape_count in regions:
            region_area = width * height
            raw_tile_count = sum(c * s for c, s in zip(shape_count, shape_sizes))
            min_region_required = (SHAPE_SIZE * SHAPE_SIZE) * sum(shape_count)

            if min_region_required <= region_area:
                # Simple square tiling is already enough
                valid += 1
            elif raw_tile_count > region_area:
                # More tiles than available
                invalid += 1
            else:
                # Could fit, but only with clever tiling
                unsure += 1

        # Assuming all unsure work out (is == 0 for input anyways)
        assert is_sample or unsure == 0
        return valid + unsure

    @staticmethod
    def solve_part02(data: ProblemInput, is_sample: bool) -> ProblemResult:
        return None


def parse(
    lines: ProblemInput,
) -> tuple[list[list[tuple[int, int]]], list[tuple[tuple[int, int], list[int]]]]:
    shape_defs = []
    shape_counts = []

    i = 0
    while i < len(lines):
        line = lines[i].strip()

        # Start of shape definition
        if len(line) == 2 and line.endswith(":"):
            i += 1
            shape = []
            for y in range(SHAPE_SIZE):
                for x in range(SHAPE_SIZE):
                    if lines[i + y][x] == "#":
                        shape.append((x, y))
            shape_defs.append(shape)
            i += SHAPE_SIZE + 1

        # Area definition
        elif ":" in line and "x" in line.split(":")[0]:
            dimensions, present_counts = line.split(":", 1)
            width, height = map(int, dimensions.strip().split("x"))
            counts = list(map(int, present_counts.strip().split()))

            shape_counts.append(((width, height), counts))
            i += 1

    return shape_defs, shape_counts
