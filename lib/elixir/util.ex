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

  @doc """
  Transposes a grid (list of lists).
  """
  @spec grid_transpose([[any()]], any()) :: [[any()]]
  def grid_transpose(grid, fill \\ nil) do
    grid |> zip_longest(fill) |> Enum.map(&Tuple.to_list/1)
  end

  @doc """
  Zips multiple enumerables together, filling shorter ones with a specified value.
  """
  @spec zip_longest([Enumerable.t()], any()) :: [tuple()]
  def zip_longest(enumerables, fill \\ nil) do
    max_length = enumerables |> Enum.map(&Enum.count/1) |> Enum.max()

    enumerables
    |> Enum.map(fn enum ->
      enum
      |> Stream.concat(Stream.cycle([fill]))
      |> Enum.take(max_length)
    end)
    |> Enum.zip()
  end
end
