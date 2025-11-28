defmodule AocLib.Util do
  @moduledoc """
  Utility functions for Advent of Code solutions.
  """

  @doc """
  Parses a list of strings to integers.
  """
  @spec parse_integers([String.t()]) :: [integer()]
  def parse_integers(strings) do
    strings
    |> Enum.map(&String.trim/1)
    |> Enum.filter(&(&1 != ""))
    |> Enum.map(&String.to_integer/1)
  end

  #   @doc """
  #   Splits a string by whitespace and parses to integers.
  #   """
  #   @spec split_integers(String.t()) :: [integer()]
  #   def split_integers(string) do
  #     string
  #     |> String.split()
  #     |> parse_integers()
  #   end

  #   @doc """
  #   Transposes a 2D grid (list of lists).
  #   """
  #   @spec transpose([[any()]]) :: [[any()]]
  #   def transpose([[] | _]), do: []

  #   def transpose(grid) do
  #     [Enum.map(grid, &hd/1) | transpose(Enum.map(grid, &tl/1))]
  #   end

  #   @doc """
  #   Counts occurrences of each element in an enumerable.
  #   Returns a map of element => count.
  #   """
  #   @spec frequencies(Enum.t()) :: %{any() => non_neg_integer()}
  #   def frequencies(enumerable) do
  #     Enum.reduce(enumerable, %{}, fn item, acc ->
  #       Map.update(acc, item, 1, &(&1 + 1))
  #     end)
  #   end

  #   @doc """
  #   Returns all unique pairs from a list.
  #   """
  #   @spec pairs([any()]) :: [{any(), any()}]
  #   def pairs(list) do
  #     for {x, i} <- Enum.with_index(list),
  #         {y, j} <- Enum.with_index(list),
  #         i < j,
  #         do: {x, y}
  #   end

  #   @doc """
  #   Returns all unique combinations of size k from a list.
  #   """
  #   @spec combinations([any()], non_neg_integer()) :: [[any()]]
  #   def combinations(_, 0), do: [[]]
  #   def combinations([], _), do: []

  #   def combinations([h | t], k) do
  #     for(combo <- combinations(t, k - 1), do: [h | combo]) ++ combinations(t, k)
  #   end

  #   @doc """
  #   Greatest common divisor.
  #   """
  #   @spec gcd(integer(), integer()) :: integer()
  #   def gcd(a, 0), do: abs(a)
  #   def gcd(a, b), do: gcd(b, rem(a, b))

  #   @doc """
  #   Least common multiple.
  #   """
  #   @spec lcm(integer(), integer()) :: integer()
  #   def lcm(a, b), do: div(abs(a * b), gcd(a, b))

  #   @doc """
  #   Manhattan distance between two points.
  #   """
  #   @spec manhattan_distance({number(), number()}, {number(), number()}) :: number()
  #   def manhattan_distance({x1, y1}, {x2, y2}) do
  #     abs(x1 - x2) + abs(y1 - y2)
  #   end
end
