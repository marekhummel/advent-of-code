defmodule Day04 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 04
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput

  @impl true
  def results do
    [
      13,
      1349,
      43,
      8277
    ]
  end

  @impl true
  def solve_part01(input, _is_sample) do
    input |> paper_rolls_map() |> work_forklifts() |> elem(0)
  end

  @impl true
  def solve_part02(input, _is_sample) do
    {paper_rolls, w, h} = paper_rolls_map(input)

    Stream.unfold(paper_rolls, fn pr ->
      case work_forklifts({pr, w, h}) do
        {0, _} -> nil
        result -> result
      end
    end)
    |> Enum.sum()
  end

  defp paper_rolls_map(input) do
    # Keep track fo paper rolls in a set of indices
    grid = ProblemInput.grid(input)
    {h, w} = {length(grid), length(hd(grid))}

    map =
      for {row, y} <- Enum.with_index(grid),
          {cell, x} <- Enum.with_index(row),
          cell == "@",
          into: MapSet.new(),
          do: {x, y}

    {map, w, h}
  end

  defp work_forklifts({paper_rolls, w, h}) do
    # Apply one round of forklift work, removing paper rolls that can be picked up
    for {x, y} <- paper_rolls, reduce: {0, paper_rolls} do
      {removed, set} ->
        neighbors =
          for iy <- max(0, y - 1)..min(h - 1, y + 1),
              ix <- max(0, x - 1)..min(w - 1, x + 1),
              do: {ix, iy}

        if Enum.count(neighbors, &MapSet.member?(paper_rolls, &1)) <= 4 do
          {removed + 1, MapSet.delete(set, {x, y})}
        else
          {removed, set}
        end
    end
  end
end
