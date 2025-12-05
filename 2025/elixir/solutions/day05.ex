defmodule Day05 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 05
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput

  @impl true
  def results do
    [
      3,
      775,
      14,
      350_684_792_662_845
    ]
  end

  @impl true
  def solve_part01(input, _is_sample) do
    {fresh_ranges, ids} = parse_input(input)

    ids |> Enum.count(fn id -> Enum.any?(fresh_ranges, fn range -> id in range end) end)
  end

  @impl true
  def solve_part02(input, _is_sample) do
    # Sort ranges to make overlap detection easier
    fresh_ranges = parse_input(input) |> elem(0) |> Enum.sort_by(& &1.first)

    # Shorten new ranges to avoid overlaps
    exclusive_ranges =
      for new_range <- fresh_ranges, reduce: [] do
        [] ->
          [new_range]

        [last_excl_range | rest] when new_range.first <= last_excl_range.last ->
          merged_range = last_excl_range.first..max(last_excl_range.last, new_range.last)
          [merged_range | rest]

        excl_ranges ->
          [new_range | excl_ranges]
      end

    exclusive_ranges |> Enum.map(&Range.size/1) |> Enum.sum()
  end

  def parse_input(input) do
    lines = ProblemInput.lines(input)

    {first_part, rest} = Enum.split_while(lines, &(&1 != ""))
    second_part = tl(rest)

    fresh_ranges =
      first_part
      |> Enum.map(&String.split(&1, "-", parts: 2))
      |> Enum.map(fn [from, to] -> String.to_integer(from)..String.to_integer(to) end)

    ingredients =
      second_part
      |> Enum.map(&String.to_integer/1)

    {fresh_ranges, ingredients}
  end
end
