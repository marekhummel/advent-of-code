defmodule Day08 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 08
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput
  alias AocLib.Util
  alias AocLib.Util.Vec3
  alias AocLib.Util.UnionFind

  @impl true
  def results do
    [
      40,
      131_150,
      25272,
      2_497_445
    ]
  end

  @impl true
  def solve_part01(input, is_sample) do
    num_lights = if(is_sample, do: 10, else: 1000)

    {jboxes, lights} = jboxes_and_lights(input)

    # Connect
    uf =
      for [a, b] <- lights |> Enum.take(num_lights), reduce: UnionFind.new(jboxes) do
        uf_acc -> UnionFind.union(uf_acc, a, b)
      end

    # Find largest 3 circuits (sets() gives root and member count)
    {circuits, _} = UnionFind.sets(uf)
    circuits |> Map.values() |> Enum.sort(:desc) |> Enum.take(3) |> Enum.product()
  end

  @impl true
  def solve_part02(input, _is_sample) do
    {jboxes, lights} = jboxes_and_lights(input)

    {_, connection} =
      Enum.reduce_while(lights, {UnionFind.new(jboxes), nil}, fn [a, b], {uf_acc, _} ->
        uf_new = UnionFind.union(uf_acc, a, b)

        {root, uf_new} = UnionFind.find(uf_new, hd(jboxes))

        case Enum.all?(jboxes, &(UnionFind.find(uf_new, &1) |> elem(0) == root)) do
          false -> {:cont, {uf_new, nil}}
          true -> {:halt, {uf_new, a.x * b.x}}
        end
      end)

    connection
  end

  @spec jboxes_and_lights(ProblemInput.t()) :: {[Vec3.t()], [[Vec3.t()]]}
  defp jboxes_and_lights(input) do
    jboxes =
      ProblemInput.lines(input)
      |> Enum.map(fn ln ->
        [x, y, z] = String.split(ln, ",", trim: true) |> Enum.map(&String.to_integer/1)
        Vec3.new(x, y, z)
      end)

    lights = jboxes |> Util.combinations(2) |> Enum.sort_by(fn [a, b] -> Vec3.dist_sq(a, b) end)

    {jboxes, lights}
  end
end
