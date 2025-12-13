defmodule Day12 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 12
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput

  @impl true
  def results do
    [
      :unsolved,
      448,
      :no_part_two,
      :no_part_two
    ]
  end

  @impl true
  def solve_part01(input, is_sample) do
    {shape_sizes, region_requirements} = input |> ProblemInput.lines() |> parse()

    {valid, _invalid, unsure} =
      for {{width, height}, present_counts} <- region_requirements, reduce: {0, 0, 0} do
        {valid, invalid, unsure} ->
          region_area = width * height

          raw_tile_count =
            Enum.zip(present_counts, shape_sizes) |> Enum.sum_by(fn {c, s} -> c * s end)

          min_region_required = 3 * 3 * Enum.sum(present_counts)

          cond do
            # Put each present in its own 3x3 area, so definitely valid
            min_region_required <= region_area ->
              {valid + 1, invalid, unsure}

            # Even if we cut all present in unit tiles, this wont fit, so definitely invalid
            raw_tile_count > region_area ->
              {valid, invalid + 1, unsure}

            # Can only fit with clever packing
            true ->
              {valid, invalid, unsure + 1}
          end
      end

    if not is_sample and unsure == 0, do: valid, else: :unsolved
  end

  @impl true
  def solve_part02(_input, _is_sample) do
    :no_part_two
  end

  defp parse(lines) do
    {regions, shapes} =
      lines
      |> Enum.chunk_by(&(&1 == ""))
      |> Enum.reject(&(length(&1) == 1 and hd(&1) == ""))
      |> List.pop_at(-1)

    shape_sizes =
      shapes
      |> Enum.map(fn shape ->
        tl(shape) |> Enum.join() |> String.graphemes() |> Enum.count(&(&1 == "#"))
      end)

    region_requirements =
      regions
      |> Enum.map(fn def ->
        [dimensions, present_counts] = String.split(def, ": ", parts: 2, trim: true)

        [width, length] = dimensions |> String.split("x") |> Enum.map(&String.to_integer/1)
        presents = present_counts |> String.split(" ") |> Enum.map(&String.to_integer/1)
        {{width, length}, presents}
      end)

    {shape_sizes, region_requirements}
  end
end
