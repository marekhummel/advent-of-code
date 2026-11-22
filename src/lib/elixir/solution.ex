defmodule AocLib.Solution do
  @moduledoc """
  The two required callable parts of a day.
  """

  alias AocLib.Types.{ProblemInput, ProblemResult}

  @type part :: (ProblemInput.t(), boolean() -> ProblemResult.t())
  @type t :: %__MODULE__{part1: part(), part2: part()}

  @enforce_keys [:part1, :part2]
  defstruct [:part1, :part2]

  @doc """
  Registers a day module after checking that both part functions exist.
  """
  @spec from(module()) :: t()
  def from(day) do
    Code.ensure_loaded!(day)

    for part <- [:part1, :part2] do
      unless function_exported?(day, part, 2) do
        raise ArgumentError, "#{inspect(day)} must export #{part}/2"
      end
    end

    %__MODULE__{part1: &day.part1/2, part2: &day.part2/2}
  end
end
