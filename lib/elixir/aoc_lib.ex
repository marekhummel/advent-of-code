defmodule AocLib do
  @moduledoc """
  Main module for Advent of Code library.
  """

  # Re-export main modules
  defdelegate new(year, solutions), to: AocLib.Runner
  defdelegate run(runner, arg, full_day, part, use_sample), to: AocLib.Runner
end
