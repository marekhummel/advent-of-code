defmodule Day07 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 07
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput

  @impl true
  def results do
    [
      21,
      1598,
      40,
      4_509_723_641_302
    ]
  end

  @impl true
  def solve_part01(input, _is_sample) do
    manifold = ProblemInput.grid(input)

    {_beams, split_count} =
      for row <- manifold, reduce: {[], 0} do
        {[], _} ->
          {[Enum.find_index(row, &(&1 == "S"))], 0}

        {beams, split_count} ->
          new_beams =
            Enum.flat_map(beams, fn beam ->
              case Enum.at(row, beam) do
                "^" -> [beam - 1, beam + 1]
                _ -> [beam]
              end
            end)

          {new_beams |> Enum.uniq(), split_count + length(new_beams) - length(beams)}
      end

    split_count
  end

  @impl true
  def solve_part02(input, _is_sample) do
    manifold = ProblemInput.grid(input)

    worlds =
      for row <- manifold, reduce: [] do
        [] ->
          [{Enum.find_index(row, &(&1 == "S")), 1}]

        worlds ->
          new_worlds =
            for {beam, world_count} <- worlds do
              case Enum.at(row, beam) do
                "^" -> [{beam - 1, world_count}, {beam + 1, world_count}]
                _ -> [{beam, world_count}]
              end
            end

          new_worlds
          |> List.flatten()
          |> Enum.group_by(fn {beam, _} -> beam end, fn {_, count} -> count end)
          |> Enum.map(fn {beam, counts} -> {beam, Enum.sum(counts)} end)
      end

    worlds |> Enum.sum_by(fn {_, count} -> count end)
  end
end
