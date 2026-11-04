defmodule Day02 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 02
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput
  alias AocLib.Util

  @impl true
  def results do
    [
      1_227_775_554,
      12_586_854_255,
      4_174_379_265,
      17_298_174_201
    ]
  end

  @impl true
  def solve_part01(input, _is_sample) do
    ProblemInput.string(input)
    |> String.split(",", trim: true)
    |> Enum.flat_map(&parse_range/1)
    |> Enum.flat_map(&find_invalid_ids(&1, 2..2))
    |> Enum.sum()
  end

  @impl true
  def solve_part02(input, _is_sample) do
    ProblemInput.string(input)
    |> String.split(",", trim: true)
    |> Enum.flat_map(&parse_range/1)
    |> Enum.reject(fn {digits, _, _} -> digits == 1 end)
    |> Enum.flat_map(fn {digits, from, to} -> find_invalid_ids({digits, from, to}, 2..digits) end)
    |> Enum.sum()
  end

  @spec parse_range(String.t()) :: [{non_neg_integer(), non_neg_integer(), non_neg_integer()}]
  defp parse_range(range_str) do
    # Parses a input range string into a list of ranges, split by length of digits
    # E.G.: "95-1234" becomes [{2, 95, 99}, {3, 100, 999}, {4, 1000, 1234}]

    [left, right] = String.split(range_str, "-", parts: 2) |> Enum.map(&String.to_integer/1)
    left_digits = Util.count_digits(left)
    right_digits = Util.count_digits(right)

    if left_digits == right_digits do
      [{left_digits, left, right}]
    else
      for digits <- left_digits..right_digits do
        range_start = if digits == left_digits, do: left, else: Util.power_of_10(digits - 1)
        range_end = if digits == right_digits, do: right, else: Util.power_of_10(digits) - 1
        {digits, range_start, range_end}
      end
    end
  end

  @spec find_invalid_ids({non_neg_integer(), non_neg_integer(), non_neg_integer()}, Range.t()) ::
          [non_neg_integer()]
  defp find_invalid_ids({num_digits, from, to}, repeat_range) do
    # Finds all invalid ids by trying repeating sequences that fit in the range

    repeat_range
    |> Enum.filter(fn r -> rem(num_digits, r) == 0 end)
    |> Enum.flat_map(fn r ->
      seq_length = div(num_digits, r)

      [seq_start, seq_end] =
        [from, to] |> Enum.map(&div(&1, Util.power_of_10(num_digits - seq_length)))

      seq_start..seq_end
      |> Enum.map(fn seq -> String.to_integer(String.duplicate(to_string(seq), r)) end)
      |> Enum.reject(fn id -> id < from or id > to end)
    end)
    |> Enum.uniq()
  end
end
