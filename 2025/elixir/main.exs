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

  def main(args) do
    runner = create_runner()

    case AocLib.Runner.parse_args(args) do
      :help ->
        print_help()

      {:ok, command, opts} ->
        AocLib.Runner.run(runner, command, opts.all, opts.part, opts.use_sample)

      {:error, message} ->
        IO.puts("Error: #{message}")
        IO.puts("Use --help for usage information")
        System.halt(1)
    end
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

  defp print_help do
    IO.puts("""
    Advent of Code 2025 - Elixir Solutions

    Usage:
      elixir main.exs [command] [options]

    Commands:
      test                 Run verification tests for all implemented solutions
      main                 Run all implemented days (ignores all options)
      day N                Run a specific day (e.g., day 1, day 15)

    Options (for day N only):
      -s, --sample         Use sample input instead of real input
      -p, --part <1|2>     Run only part 1 or part 2 (REQUIRED unless --all)
      -a, --all            Run both parts (overrides --part)
      -h, --help           Show this help message

    Examples:
      elixir main.exs test
      elixir main.exs main
      elixir main.exs day 1 --sample --all
      elixir main.exs day 1 --part 1
      elixir main.exs day 5 --part 2 --sample
    """)
  end
end

# Run the main or test function based on arguments
case System.argv() do
  ["test" | _rest] -> Main.test()
  args -> Main.main(args)
end
