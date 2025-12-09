defmodule Day09 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 09
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput
  alias AocLib.Util

  @impl true
  def results do
    [
      50,
      4_782_896_435,
      24,
      1_540_060_480
    ]
  end

  @impl true
  def solve_part01(input, _is_sample) do
    input |> red_tiles() |> Util.combinations(2) |> Enum.map(&area/1) |> Enum.max()
  end

  @impl true
  def solve_part02(input, _is_sample) do
    red_tiles = red_tiles(input)

    edges =
      Enum.chunk_every(red_tiles ++ [hd(red_tiles)], 2, 1, :discard) |> Enum.map(&normalize/1)

    for pair <- red_tiles |> Util.combinations(2), reduce: 0 do
      best ->
        a = area(pair)
        if a > best and not intersects(edges, normalize(pair)), do: a, else: best
    end
  end

  @type xy :: {integer(), integer(), integer(), integer()}

  @spec red_tiles(ProblemInput.t()) :: [[integer()]]
  defp red_tiles(input) do
    input
    |> ProblemInput.lines()
    |> Enum.map(fn ln -> String.split(ln, ",") |> Enum.map(&String.to_integer/1) end)
  end

  @spec area([[integer()]]) :: integer()
  defp area([[x1, y1], [x2, y2]]), do: (abs(x2 - x1) + 1) * (abs(y2 - y1) + 1)

  @spec normalize([[integer()]]) :: xy()
  defp normalize([[x1, y1], [x2, y2]]),
    do: {min(x1, x2), max(x1, x2), min(y1, y2), max(y1, y2)}

  @spec intersects([xy()], xy()) :: boolean()
  defp intersects([], _rect), do: false

  defp intersects([{exmin, exmax, eymin, eymax} | rest], {rxmin, rxmax, rymin, rymax} = rect) do
    intersects = rxmin < exmax and exmin < rxmax and rymin < eymax and eymin < rymax
    intersects or intersects(rest, rect)
  end
end
