defmodule Main do
  @moduledoc """
  Main entry point for Advent of Code 2025 solutions.
  """

  alias AocLib.Solution

  def main(args) do
    AocLib.Runner.parse_and_run(create_runner(), args)
  end

  def create_runner do
    solutions = %{
      # aoc:solutions
      1 => Solution.from(Day01),
      2 => Solution.from(Day02),
      3 => Solution.from(Day03),
      4 => Solution.from(Day04),
      5 => Solution.from(Day05),
      6 => Solution.from(Day06),
      7 => Solution.from(Day07),
      8 => Solution.from(Day08),
      9 => Solution.from(Day09),
      10 => Solution.from(Day10),
      11 => Solution.from(Day11),
      12 => Solution.from(Day12)
    }

    AocLib.Runner.new(2025, solutions)
  end
end
