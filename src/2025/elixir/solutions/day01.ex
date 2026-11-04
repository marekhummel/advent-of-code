defmodule Day01 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 01
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput
  alias AocLib.Util

  @impl true
  def results do
    [
      3,
      1177,
      6,
      6768
    ]
  end

  @dial_size 100

  @impl true
  def solve_part01(input, _is_sample) do
    instructions = parse_instructions(input)

    positions =
      Enum.scan(instructions, {50, 0, 0}, fn {dir, steps}, {current, _, _} ->
        apply_rot(current, steps, dir)
      end)

    password =
      positions
      |> Enum.map(&elem(&1, 1))
      |> Enum.sum()

    password
  end

  @impl true
  def solve_part02(input, _is_sample) do
    instructions = parse_instructions(input)

    positions =
      Enum.scan(instructions, {50, 0, 0}, fn {dir, steps}, {current, _, _} ->
        apply_rot(current, steps, dir)
      end)

    password =
      positions
      |> Enum.map(fn {_pos, on_zero, over_zero} -> on_zero + over_zero end)
      |> Enum.sum()

    password
  end

  @spec parse_instructions(ProblemInput.t()) :: [{String.t(), integer()}]
  defp parse_instructions(input) do
    ProblemInput.lines(input)
    |> Enum.map(fn line ->
      {dir, steps} = String.split_at(line, 1)
      {dir, String.to_integer(steps)}
    end)
  end

  @spec apply_rot(integer(), integer(), String.t()) :: {integer(), integer(), integer()}
  defp apply_rot(current, steps, direction) do
    dest =
      case direction do
        "L" -> current - steps
        "R" -> current + steps
      end

    over_zero = abs(floor(dest / @dial_size))

    dest = Integer.mod(dest, @dial_size)

    over_zero =
      if (direction == "L" and current == 0) or (direction == "R" and dest == 0) do
        max(over_zero - 1, 0)
      else
        over_zero
      end

    {dest, Util.bool_to_int(dest == 0), over_zero}
  end
end
