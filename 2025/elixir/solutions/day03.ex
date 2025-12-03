defmodule Day03 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 03
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput

  @impl true
  def results do
    [
      357,
      17_095,
      3_121_910_778_619,
      168_794_698_570_517
    ]
  end

  @impl true
  def solve_part01(input, _is_sample) do
    input |> banks_from_line() |> Enum.map(&largest_joltage(&1, 2)) |> Enum.sum()
  end

  @impl true
  def solve_part02(input, _is_sample) do
    input |> banks_from_line() |> Enum.map(&largest_joltage(&1, 12)) |> Enum.sum()
  end

  @spec banks_from_line(ProblemInput.t()) :: [[non_neg_integer()]]
  defp banks_from_line(input) do
    for line <- ProblemInput.lines(input) do
      Enum.map(String.graphemes(line), &String.to_integer/1)
    end
  end

  @spec largest_joltage([non_neg_integer()], non_neg_integer()) :: non_neg_integer()
  defp largest_joltage(bank, num_batteries) do
    # Finds the largest joltage by selecting num_batteries from the bank
    for battery <- 1..num_batteries, reduce: {0, 0} do
      {pos, joltage} ->
        possibles = pos..(length(bank) - (num_batteries - battery) - 1)
        best = possibles |> Enum.max_by(fn next -> Enum.at(bank, next) end)
        {best + 1, joltage * 10 + Enum.at(bank, best)}
    end
    |> elem(1)
  end
end
