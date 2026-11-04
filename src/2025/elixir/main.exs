# Get directory of this script file
script_dir = __DIR__

# Add lib directory to load path
Code.append_path(Path.join([script_dir, "..", "..", "lib", "elixir"]))

# Import library modules
Code.require_file(Path.join([script_dir, "..", "..", "lib", "elixir", "types.ex"]))
Code.require_file(Path.join([script_dir, "..", "..", "lib", "elixir", "solution.ex"]))
Code.require_file(Path.join([script_dir, "..", "..", "lib", "elixir", "runner.ex"]))
Code.require_file(Path.join([script_dir, "..", "..", "lib", "elixir", "util.ex"]))
Code.require_file(Path.join([script_dir, "..", "..", "lib", "elixir", "aoc_lib.ex"]))

# Import solution modules
Code.require_file(Path.join([script_dir, "solutions", "day01.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day02.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day03.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day04.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day05.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day06.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day07.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day08.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day09.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day10.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day11.ex"]))
Code.require_file(Path.join([script_dir, "solutions", "day12.ex"]))
# aoc:imports

defmodule Main do
  @moduledoc """
  Main entry point for Advent of Code 2025 solutions.
  """

  def main(args) do
    AocLib.Runner.parse_and_run(create_runner(), args)
  end

  def create_runner do
    solutions = %{
      # aoc:solutions
      1 => Day01,
      2 => Day02,
      3 => Day03,
      4 => Day04,
      5 => Day05,
      6 => Day06,
      7 => Day07,
      8 => Day08,
      9 => Day09,
      10 => Day10,
      11 => Day11,
      12 => Day12
    }

    AocLib.Runner.new(2025, solutions)
  end
end

Main.main(System.argv())
