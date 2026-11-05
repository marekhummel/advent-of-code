defmodule AocLib.Runner do
  @moduledoc """
  Runner for executing a single Advent of Code solution and printing its answer.
  """

  alias AocLib.Types.{ProblemInput, ProblemResult}

  defstruct [:year, :solutions]

  @type t :: %__MODULE__{
          year: non_neg_integer(),
          solutions: %{pos_integer() => module()}
        }

  @doc """
  Creates a new runner for a given year with a map of day numbers to solution
  modules. Registration order does not matter, and days may be omitted.
  """
  @spec new(non_neg_integer(), %{pos_integer() => module()}) :: t()
  def new(year, solutions), do: %__MODULE__{year: year, solutions: solutions}

  @doc """
  Parses DAY, PART and MODE (real or sample), then runs the selected case.
  Prints the answer to stdout. Invalid arguments exit with code 2; execution
  errors are raised and reported on stderr. Missing days, missing parts and
  results :unsolved, :no_sample and :no_part_two are informational and exit
  successfully.
  """
  @spec parse_and_run(t(), [String.t()]) :: :ok
  def parse_and_run(runner, args) do
    case args do
      [day, part, mode] when part in ["1", "2"] and mode in ["real", "sample"] ->
        case Integer.parse(day) do
          {day, ""} when day in 1..25 ->
            run(runner, day, String.to_integer(part), mode == "sample")

          _ ->
            usage()
        end

      _ ->
        usage()
    end
  end

  defp usage do
    IO.puts(:stderr, "Usage: main.exs DAY PART MODE (real|sample)")
    System.halt(2)
  end

  defp run(runner, day, part, sample) do
    solution = Map.get(runner.solutions, day)
    solve = if part == 1, do: :solve_part01, else: :solve_part02

    cond do
      solution == nil ->
        IO.puts("No solution implemented for this day")

      not function_exported?(solution, solve, 2) ->
        IO.puts("No solution implemented for this part")

      true ->
        run_solution(runner, day, part, sample, solution, solve)
    end
  end

  defp run_solution(runner, day, part, sample, solution, solve) do
    prefix = if sample, do: "sample", else: "input"
    stem = "inputs/#{runner.year}/#{prefix}#{String.pad_leading(to_string(day), 2, "0")}"
    filename = if File.exists?(stem <> ".txt"), do: stem <> ".txt", else: "#{stem}_#{part}.txt"

    input =
      case ProblemInput.read(filename) do
        {:ok, input} -> input
        {:error, reason} -> raise "Could not read #{filename}: #{:file.format_error(reason)}"
      end

    result =
      case apply(solution, solve, [input, sample]) do
        nil -> :unsolved
        result -> result
      end

    if is_atom(result) and result not in [:unsolved, :no_sample, :no_part_two],
      do: raise("Solution unavailable: #{result}")

    IO.puts(ProblemResult.format(result))
  end
end
