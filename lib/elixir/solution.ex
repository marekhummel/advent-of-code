defmodule AocLib.Solution do
  @moduledoc """
  Behaviour for Advent of Code solution implementations.
  """

  alias AocLib.Types.{ProblemInput, ProblemResult}

  @type part :: 1 | 2
  @type is_sample :: boolean()

  @doc """
  Returns the expected results for testing: [part1_sample, part1_real, part2_sample, part2_real]
  """
  @callback results() :: [ProblemResult.t()]

  @doc """
  Solves part 1 of the problem.
  """
  @callback solve_part01(ProblemInput.t(), is_sample()) :: ProblemResult.t()

  @doc """
  Solves part 2 of the problem.
  """
  @callback solve_part02(ProblemInput.t(), is_sample()) :: ProblemResult.t()

  @doc """
  Solves the problem based on the part number.
  Returns a tuple of {result, duration_in_seconds}.
  """
  @spec solve(module(), ProblemInput.t(), part(), is_sample()) ::
          {ProblemResult.t(), float()}
  def solve(module, input, part, is_sample) do
    start_time = System.monotonic_time(:nanosecond)

    result =
      case part do
        1 -> module.solve_part01(input, is_sample)
        2 -> module.solve_part02(input, is_sample)
        _ -> raise "Invalid part number: #{part}"
      end

    end_time = System.monotonic_time(:nanosecond)
    duration = (end_time - start_time) / 1_000_000_000

    {result, duration}
  end
end
