defmodule AocLib.Runner do
  @moduledoc """
  Runner for executing a single Advent of Code solution and printing its answer.
  """

  alias AocLib.Types.ProblemInput

  defstruct [:year, :solutions]

  @type t :: %__MODULE__{
          year: non_neg_integer(),
          solutions: [module() | nil]
        }

  @doc """
  Creates a new runner for a given year with a list of solution modules.
  """
  @spec new(non_neg_integer(), [module() | nil]) :: t()
  def new(year, solutions), do: %__MODULE__{year: year, solutions: solutions}

  @doc """
  Parses DAY, PART and an optional --sample flag, then runs the selected case.
  Prints the answer to stdout. Invalid arguments exit with code 2; execution
  errors are raised and reported on stderr.
  """
  @spec parse_and_run(t(), [String.t()]) :: :ok
  def parse_and_run(runner, args) do
    case args do
      [day, part | options] when part in ["1", "2"] and options in [[], ["--sample"]] ->
        case Integer.parse(day) do
          {day, ""} when day in 1..25 ->
            run(runner, day, String.to_integer(part), options == ["--sample"])

          _ ->
            usage()
        end

      _ ->
        usage()
    end
  end

  defp usage do
    IO.puts(:stderr, "Usage: main.exs DAY PART [--sample]")
    System.halt(2)
  end

  defp run(runner, day, part, sample) do
    solution = Enum.at(runner.solutions, day - 1)
    if solution == nil, do: raise("No solution for #{runner.year} day #{day}")
    prefix = if sample, do: "sample", else: "input"
    stem = "inputs/#{runner.year}/#{prefix}#{String.pad_leading(to_string(day), 2, "0")}"
    filename = if File.exists?(stem <> ".txt"), do: stem <> ".txt", else: "#{stem}_#{part}.txt"

    input =
      case ProblemInput.read(filename) do
        {:ok, input} -> input
        {:error, reason} -> raise "Could not read #{filename}: #{:file.format_error(reason)}"
      end

    result =
      case part do
        1 -> solution.solve_part01(input, sample)
        2 -> solution.solve_part02(input, sample)
      end

    if is_atom(result), do: raise("Solution unavailable: #{result}")
    IO.puts(result)
  end
end
