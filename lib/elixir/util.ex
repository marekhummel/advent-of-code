defmodule AocLib.Util do
  @moduledoc """
  Utility functions for Advent of Code solutions.
  """

  @doc """
  Counts the number of digits in a non-negative integer.
  """
  @spec count_digits(non_neg_integer()) :: non_neg_integer()
  def count_digits(n) when n >= 0 do
    ceil(:math.log10(n + 1))
  end

  @doc """
  Computes 10 raised to the power of `exp`.
  """
  @spec power_of_10(non_neg_integer()) :: non_neg_integer()
  def power_of_10(exp), do: trunc(:math.pow(10, exp))
end
