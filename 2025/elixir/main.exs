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

defmodule Main do
  @moduledoc """
  Main entry point for Advent of Code 2025 solutions.
  """

  # Configuration flags
  @all false
  @part 1
  @use_sample true

  def main do
    runner = create_runner()
    arg = System.argv() |> List.first()
    AocLib.Runner.run(runner, arg, @all, @part, @use_sample)
  end

  def test do
    runner = create_runner()

    if AocLib.Runner.verify_solutions(runner) do
      IO.puts("\n✓ All tests passed!")
      System.halt(0)
    else
      IO.puts("\n✗ Some tests failed")
      System.halt(1)
    end
  end

  def create_runner do
    solutions = [
      Day01,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil,
      nil
    ]

    AocLib.Runner.new(2025, solutions)
  end
end

# Run the main or test function based on arguments
case System.argv() do
  ["test"] -> Main.test()
  _ -> Main.main()
end
