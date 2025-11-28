defmodule AocLib.Runner do
  @moduledoc """
  Runner for executing Advent of Code solutions.
  """

  alias AocLib.Solution
  alias AocLib.Types.{ProblemInput, ProblemResult}

  defstruct [:year, :solutions]

  @type t :: %__MODULE__{
          year: non_neg_integer(),
          solutions: [module() | nil]
        }

  @sample_str ["real", "samp"]

  @doc """
  Creates a new runner for a given year with a list of solution modules.
  """
  @spec new(non_neg_integer(), [module() | nil]) :: t()
  def new(year, solutions) do
    %__MODULE__{year: year, solutions: solutions}
  end

  @doc """
  Runs the solutions based on command line arguments.
  """
  @spec run(t(), String.t() | nil, boolean(), 1 | 2, boolean()) :: :ok
  def run(runner, arg, full_day, part, use_sample) do
    case arg do
      nil ->
        IO.puts("Usage: elixir main.exs <day>")

      "main" ->
        run_full_year(runner)

      "day" <> day_str ->
        case Integer.parse(day_str) do
          {day, ""} when day >= 1 and day <= 25 ->
            if full_day do
              run_day(runner, day)
            else
              run_single(runner, day, part, use_sample)
            end

          _ ->
            IO.puts("Argument should have format dayXX: #{arg}")
        end

      _ ->
        IO.puts("Argument should be 'main' or 'dayXX': #{arg}")
    end
  end

  @doc """
  Verifies that a solution matches the expected result.
  """
  @spec verify_solution(t(), non_neg_integer(), 1 | 2, boolean()) ::
          :ok | {:error, String.t()}
  def verify_solution(runner, day, part, use_sample) do
    solution = Enum.at(runner.solutions, day - 1)

    if solution == nil do
      {:error, "No solution implemented for day #{day}"}
    else
      expected_index = (part - 1) * 2 + if(use_sample, do: 0, else: 1)
      expected = Enum.at(solution.results(), expected_index)

      case get_result(runner, day, part, use_sample) do
        {:ok, {result, _duration}} ->
          if ProblemResult.equal?(result, expected) do
            :ok
          else
            {:error,
             "Got #{ProblemResult.format(result)}, expected #{ProblemResult.format(expected)}"}
          end

        {:error, reason} ->
          {:error, "Error: #{reason}"}
      end
    end
  end

  @doc """
  Verifies all solutions against expected results.
  Returns true if all tests pass.
  """
  @spec verify_solutions(t()) :: boolean()
  def verify_solutions(runner) do
    IO.puts("\n----------")

    success =
      runner.solutions
      |> Enum.with_index(1)
      |> Enum.reduce(true, fn {solution, day}, acc ->
        if solution != nil do
          # Check all 4 combinations for this day
          day_success =
            for part <- [1, 2], use_sample <- [true, false], reduce: true do
              day_acc ->
                sample_label = if use_sample, do: "s", else: "r"

                case get_result(runner, day, part, use_sample) do
                  {:ok, {result, _duration}} ->
                    expected_index = (part - 1) * 2 + if(use_sample, do: 0, else: 1)
                    expected = Enum.at(solution.results(), expected_index)

                    IO.write(
                      "Testing D#{String.pad_leading(Integer.to_string(day), 2, "0")} P#{part} '#{sample_label}': "
                    )

                    if ProblemResult.equal?(result, expected) do
                      IO.puts("PASSED")
                      day_acc
                    else
                      IO.puts(
                        "FAILED: Got #{ProblemResult.format(result)}, expected #{ProblemResult.format(expected)}"
                      )

                      false
                    end

                  {:error, :missing_solution} ->
                    day_acc

                  {:error, reason} ->
                    IO.write(
                      "Testing D#{String.pad_leading(Integer.to_string(day), 2, "0")} P#{part} '#{sample_label}': "
                    )

                    IO.puts("ERROR RESULT (#{reason})")
                    false
                end
            end

          acc and day_success
        else
          acc
        end
      end)

    IO.puts("----------")
    success
  end

  defp run_full_year(runner) do
    IO.puts("")

    {total_elapsed, _} =
      :timer.tc(fn ->
        runner.solutions
        |> Enum.with_index(1)
        |> Enum.each(fn {solution, day} ->
          if solution != nil do
            IO.puts("Day #{String.pad_leading(Integer.to_string(day), 2, "0")}:")

            day_elapsed =
              for part <- [1, 2],
                  use_sample <- [true, false],
                  reduce: 0.0 do
                acc ->
                  case get_result(runner, day, part, use_sample) do
                    {:ok, {result, elapsed}} ->
                      sample_idx = if use_sample, do: 1, else: 0

                      IO.puts(
                        "  P#{part} #{Enum.at(@sample_str, sample_idx)}:  #{ProblemResult.format(result)}"
                      )

                      acc + elapsed

                    {:error, _reason} ->
                      sample_idx = if use_sample, do: 1, else: 0
                      IO.puts("  P#{part} #{Enum.at(@sample_str, sample_idx)}:  <Error>")
                      acc
                  end
              end

            IO.puts("  > Runtime: #{:io_lib.format("~.4f", [day_elapsed])}s\n")
          end
        end)
      end)

    total_secs = total_elapsed / 1_000_000
    IO.puts("\n\nTotal Runtime: #{:io_lib.format("~.4f", [total_secs])}s")
  end

  defp run_day(runner, day) do
    solution = Enum.at(runner.solutions, day - 1)

    if solution == nil do
      IO.puts(
        "No solution implemented for day #{String.pad_leading(Integer.to_string(day), 2, "0")} in year #{runner.year}"
      )
    else
      day_elapsed =
        for part <- [1, 2],
            use_sample <- [true, false],
            reduce: 0.0 do
          acc ->
            case get_result(runner, day, part, use_sample) do
              {:ok, {result, elapsed}} ->
                sample_idx = if use_sample, do: 1, else: 0

                IO.puts(
                  "P#{part} #{Enum.at(@sample_str, sample_idx)} in #{:io_lib.format("~10.4f", [elapsed])}s:    #{ProblemResult.format(result)}"
                )

                acc + elapsed

              {:error, reason} ->
                sample_idx = if use_sample, do: 1, else: 0
                IO.puts("P#{part} #{Enum.at(@sample_str, sample_idx)}:  <Error: #{reason}>")
                acc
            end
        end

      IO.puts("\nTotal Runtime: #{:io_lib.format("~.4f", [day_elapsed])}s")
    end
  end

  defp run_single(runner, day, part, use_sample) do
    case get_result(runner, day, part, use_sample) do
      {:ok, {result, elapsed}} ->
        sample_str = if use_sample, do: "samp", else: "real"

        IO.puts(
          "Day #{String.pad_leading(Integer.to_string(day), 2, "0")} / part #{part} / Data '#{sample_str}' => #{:io_lib.format("~.4f", [elapsed])}s"
        )

        IO.puts(ProblemResult.format(result))

      {:error, reason} ->
        IO.puts("Error: #{reason}")
    end
  end

  defp get_input(runner, day, part, use_sample) do
    base_filename = if use_sample, do: "sample", else: "input"
    day_str = String.pad_leading(Integer.to_string(day), 2, "0")
    filename = "#{runner.year}/inputs/#{base_filename}#{day_str}.txt"

    case ProblemInput.read(filename) do
      {:ok, input} ->
        {:ok, input}

      {:error, :enoent} ->
        # Try versioned filename
        versioned_filename = "#{runner.year}/inputs/#{base_filename}#{day_str}_#{part}.txt"

        case ProblemInput.read(versioned_filename) do
          {:ok, input} -> {:ok, input}
          {:error, _} -> {:error, :no_input}
        end

      {:error, reason} ->
        {:error, reason}
    end
  end

  defp get_result(runner, day, part, use_sample) do
    solution = Enum.at(runner.solutions, day - 1)

    if solution == nil do
      {:error, :missing_solution}
    else
      case get_input(runner, day, part, use_sample) do
        {:ok, input} ->
          {result, duration} = Solution.solve(solution, input, part, use_sample)
          {:ok, {result, duration}}

        {:error, :no_input} ->
          {:ok, {:no_input, 0.0}}

        {:error, reason} ->
          {:error, reason}
      end
    end
  end
end
