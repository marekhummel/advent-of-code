from functools import reduce
from itertools import zip_longest
from typing import NamedTuple

from lib.python import ProblemInput, ProblemResult, Solution


class MapEntry(NamedTuple):
    dst: int
    src: int
    len: int


Seed = int
Map = list[MapEntry]


class SeedRange(NamedTuple):
    start: int
    len: int


class Solution05(Solution):
    @staticmethod
    def results() -> list[ProblemResult]:
        return [35, 379811651, 46, 27992443]

    @staticmethod
    def solve_part01(data: ProblemInput, is_sample: bool) -> ProblemResult:
        seeds, maps = parse(data)
        return min(reduce(apply_map, maps, s) for s in seeds)

    @staticmethod
    def solve_part02(data: ProblemInput, is_sample: bool) -> ProblemResult:
        seeds, maps = parse2(data)
        return min(sr.start for s in seeds for sr in reduce(apply_map2, maps, [s]))


def parse(lines: ProblemInput) -> tuple[list[Seed], list[Map]]:
    seeds = [int(s) for s in lines[0].removeprefix("seeds: ").split()]
    maps = []
    current_map: list[MapEntry] = []
    for line in lines[2:]:
        if line.isspace():
            maps.append(current_map)
            current_map = []
            continue

        if line.endswith("map:\n"):
            continue

        d, s, l = [int(n) for n in line.split()]
        current_map.append(MapEntry(d, s, l))

    maps.append(current_map)
    return seeds, maps


def parse2(lines: ProblemInput) -> tuple[list[SeedRange], list[Map]]:
    seeds, maps = parse(lines)
    seed_ranges = [SeedRange(*pair) for pair in zip_longest(*[iter(seeds)] * 2)]  # type: ignore
    return seed_ranges, maps


def apply_map(value: Seed, m: Map) -> Seed:
    return next(
        (
            entry.dst + (value - entry.src)
            for entry in m
            if value in range(entry.src, entry.src + entry.len)
        ),
        value,
    )


def apply_map2(values: list[SeedRange], m: Map) -> list[SeedRange]:
    mapped_values = []
    remaining_values = values
    for value in remaining_values:
        for entry in m:
            entry_range = range(entry.src, entry.src + entry.len)
            init_fits = value.start in entry_range
            tail_fits = value.start + value.len - 1 in entry_range

            if init_fits and tail_fits:
                # Whole range can be mapped
                mapped_values.append(SeedRange(entry.dst + (value.start - entry.src), value.len))
                break
            elif init_fits:
                # Two ranges, start fits in current map entry
                mapped_len = (entry.src + entry.len) - value.start
                mapped_values.append(SeedRange(entry.dst + (value.start - entry.src), mapped_len))
                remaining_values.append(
                    SeedRange(value.start + mapped_len, value.len - mapped_len)
                )
                break
            elif tail_fits:
                # Two ranges, end fits in current map entry
                mapped_len = (value.start + value.len) - entry.src
                mapped_values.append(SeedRange(entry.dst, mapped_len))
                remaining_values.append(SeedRange(value.start, value.len - mapped_len))
                break
        else:
            mapped_values.append(value)

    return mapped_values
