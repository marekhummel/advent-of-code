defmodule AocLib.Solution do
  @moduledoc """
  Behaviour for Advent of Code solution implementations.
  """

  alias AocLib.Types.{ProblemInput, ProblemResult}

  @type part :: 1 | 2
  @type is_sample :: boolean()

  @doc """
  Solves part 1 of the problem.
  """
  @callback solve_part01(ProblemInput.t(), is_sample()) :: ProblemResult.t()

  @doc """
  Solves part 2 of the problem.
  """
  @callback solve_part02(ProblemInput.t(), is_sample()) :: ProblemResult.t()

  @optional_callbacks solve_part01: 2, solve_part02: 2
end
