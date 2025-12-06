defmodule Day06 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 06
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput
  alias AocLib.Util

  @impl true
  def results do
    [
      4_277_556,
      7_644_505_810_277,
      3_263_827,
      12_841_228_084_455
    ]
  end

  @impl true
  def solve_part01(input, _is_sample) do
    ProblemInput.lines(input)
    |> Enum.map(&String.split/1)
    |> Util.grid_transpose()
    |> Enum.map(&List.pop_at(&1, -1))
    |> compute_total()
  end

  @impl true
  def solve_part02(input, _is_sample) do
    columns = input |> ProblemInput.grid() |> Util.grid_transpose(" ")

    problems =
      columns
      |> Enum.map(&List.pop_at(&1, -1))
      |> Enum.chunk_by(fn {_, num_col} -> Enum.all?(num_col, &(&1 == " ")) end)
      |> Enum.map(&Enum.unzip/1)
      |> Enum.map(fn {op_list, arg_list} ->
        {
          op_list |> List.first(),
          arg_list |> Enum.map(&Enum.join/1) |> Enum.map(&String.trim/1)
        }
      end)
      |> Enum.reject(fn {op, _} -> op == " " end)

    problems |> compute_total()
  end

  @spec compute_total([{String.t(), [String.t()]}]) :: integer()
  defp compute_total(problems) do
    problems
    |> Enum.map(fn
      {"+", args} -> args |> Enum.map(&String.to_integer/1) |> Enum.sum()
      {"*", args} -> args |> Enum.map(&String.to_integer/1) |> Enum.product()
    end)
    |> Enum.sum()
  end
end
