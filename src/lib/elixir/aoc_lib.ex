defmodule AocLib do
  @moduledoc """
  Main module for Advent of Code library.
  """

  # Re-export main modules
  defdelegate new(year, solutions), to: AocLib.Runner
  defdelegate parse_and_run(runner, args), to: AocLib.Runner
end
