defmodule Day10 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 10
  """

  @behaviour AocLib.Solution

  import Bitwise
  alias AocLib.Types.ProblemInput
  alias AocLib.Util

  @impl true
  def results do
    [
      7,
      422,
      :unsolved,
      :unsolved
    ]
  end

  @impl true
  def solve_part01(input, _is_sample) do
    # Note: All buttons and patterns are unsigned ints, as we either have to press a button never or once
    # In the pattern we use left as the LSB, to make button presses easier
    machines = input |> ProblemInput.lines() |> Enum.map(&parse_line/1)
    machines |> Enum.map(&light_config/1) |> Enum.sum()
  end

  @impl true
  def solve_part02(_input, _is_sample) do
    # There is no ILP solver library for elixir, any BFS / DFS solutions were too slow
    # Check python for solution using PuLP
    :unsolved
  end

  defp parse_line(line) do
    # Split
    parts = String.split(line, " ", trim: true)
    [target_diagram | rest] = parts
    {joltages_str, buttons_strs} = List.pop_at(rest, -1)

    # Target (between []), returned as number
    target_diagram = target_diagram |> String.trim_leading("[") |> String.trim_trailing("]")
    target_chars = target_diagram |> String.graphemes() |> Enum.reverse()

    target =
      target_chars |> Enum.map(&Util.bool_to_int(&1 == "#")) |> Enum.reduce(0, &(&2 * 2 + &1))

    # Buttons (between ()), returned as index list and value
    buttons =
      Enum.map(buttons_strs, fn button_str ->
        button_str = button_str |> String.trim_leading("(") |> String.trim_trailing(")")
        button_pos = button_str |> String.split(",") |> Enum.map(&String.to_integer/1)
        button_value = button_pos |> Enum.map(fn pos -> 1 <<< pos end) |> Enum.sum()
        {button_pos, button_value}
      end)

    # Joltages (between {})
    joltages_str = joltages_str |> String.trim_leading("{") |> String.trim_trailing("}")
    joltages = joltages_str |> String.split(",") |> Enum.map(&String.to_integer/1)

    {target, buttons, joltages}
  end

  defp light_config({target, buttons, _joltages}) do
    Enum.find(1..length(buttons), fn length ->
      buttons
      |> Enum.map(fn {_, btn_value} -> btn_value end)
      |> Util.combinations(length)
      |> Enum.find(fn combo -> Enum.reduce(combo, 0, &Bitwise.bxor/2) == target end)
    end)
  end
end
