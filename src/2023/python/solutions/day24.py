from lib.python import ProblemInput, ProblemResult, Solution


class Solution24(Solution):
    @staticmethod
    def results() -> list[ProblemResult]:
        return [None, None, 47, 880547248556435]

    @staticmethod
    def solve_part01(data: ProblemInput, is_sample: bool) -> ProblemResult:
        return None

    @staticmethod
    def solve_part02(data: ProblemInput, is_sample: bool) -> ProblemResult:
        hailstones = parse(data)

        # Check rust version to understand terms
        p0, p1, p2 = (hailstones[0][0], hailstones[1][0], hailstones[2][0])
        q0, q1, q2 = (hailstones[0][1], hailstones[1][1], hailstones[2][1])

        a = [
            [0, (-q0[2] + q1[2]), (q0[1] - q1[1]), 0, (p0[2] - p1[2]), (-p0[1] + p1[1])],
            [(q0[2] - q1[2]), 0, (-q0[0] + q1[0]), (-p0[2] + p1[2]), 0, (p0[0] - p1[0])],
            [(-q0[1] + q1[1]), (q0[0] - q1[0]), 0, (p0[1] - p1[1]), (-p0[0] + p1[0]), 0],
            [0, (-q0[2] + q2[2]), (q0[1] - q2[1]), 0, (p0[2] - p2[2]), (-p0[1] + p2[1])],
            [(q0[2] - q2[2]), 0, (-q0[0] + q2[0]), (-p0[2] + p2[2]), 0, (p0[0] - p2[0])],
            [(-q0[1] + q2[1]), (q0[0] - q2[0]), 0, (p0[1] - p2[1]), (-p0[0] + p2[0]), 0],
        ]
        b = [
            p0[2] * q0[1] - p0[1] * q0[2] - p1[2] * q1[1] + p1[1] * q1[2],
            p0[0] * q0[2] - p0[2] * q0[0] - p1[0] * q1[2] + p1[2] * q1[0],
            p0[1] * q0[0] - p0[0] * q0[1] - p1[1] * q1[0] + p1[0] * q1[1],
            p0[2] * q0[1] - p0[1] * q0[2] - p2[2] * q2[1] + p2[1] * q2[2],
            p0[0] * q0[2] - p0[2] * q0[0] - p2[0] * q2[2] + p2[2] * q2[0],
            p0[1] * q0[0] - p0[0] * q0[1] - p2[1] * q2[0] + p2[0] * q2[1],
        ]

        x = solve_cramer(a, b)
        # print(x)
        return sum(x[:3])


def parse(lines: ProblemInput) -> list[tuple[tuple[int, int, int], tuple[int, int, int]]]:
    hailstones = []
    for l in lines:
        pos_str, vel_str = l.split("@")
        pos = tuple(int(p.strip()) for p in pos_str.split(","))
        vel = tuple(int(v.strip()) for v in vel_str.split(","))
        hailstones.append((pos, vel))

    return hailstones  # type: ignore


def det(a: list[list[int]]) -> int:
    if len(a) == 2:
        return a[0][0] * a[1][1] - a[1][0] * a[0][1]

    # Laplace expansion along first row
    val = 0
    sign = 1
    for c in range(len(a)):
        factor = a[0][c]
        submat = [r[:c] + r[c + 1 :] for r in a[1:]]
        val += sign * factor * det(submat)
        sign = -sign

    return val


def solve_cramer(a: list[list[int]], b: list[int]) -> list[int]:
    det_a = det(a)

    x = []
    for i in range(len(b)):
        temp_matrix = [r[:i] + [b[ri]] + r[i + 1 :] for ri, r in enumerate(a)]
        x.append(det(temp_matrix) // det_a)

    return x
