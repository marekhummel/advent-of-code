defmodule Day11 do
  @moduledoc """
  Solution for Advent of Code 2025 - Day 11
  """

  @behaviour AocLib.Solution

  alias AocLib.Types.ProblemInput

  @impl true
  def results do
    [
      5,
      511,
      2,
      458_618_114_529_380
    ]
  end

  @impl true
  def solve_part01(input, _is_sample) do
    machines = parse(input)
    {paths, _cache} = count_paths("you", "out", [], machines, %{})
    paths
  end

  @impl true
  def solve_part02(input, _is_sample) do
    machines = parse(input)
    {path, _cache} = count_paths("svr", "out", ["dac", "fft"], machines, %{})
    path
  end

  @spec parse(ProblemInput.t()) :: map()
  defp parse(input) do
    for connection <- ProblemInput.lines(input), into: %{} do
      [from, to] = String.split(connection, ":", parts: 2)
      {from, to |> String.split(" ", trim: true)}
    end
  end

  @spec count_paths(String.t(), String.t(), [String.t()], map(), map()) ::
          {non_neg_integer(), map()}
  defp count_paths(target, target, [], _graph, cache), do: {1, cache}
  defp count_paths(target, target, _stops, _graph, cache), do: {0, cache}

  defp count_paths(machine, _target, stops, _graph, cache)
       when is_map_key(cache, {machine, stops}), do: {Map.get(cache, {machine, stops}), cache}

  defp count_paths(machine, target, stops, graph, cache) do
    updated_stops = List.delete(stops, machine)

    {paths, updated_cache} =
      for next <- Map.get(graph, machine, []), reduce: {0, cache} do
        {acc, c} ->
          {paths_from_next, new_cache} =
            count_paths(next, target, updated_stops, graph, c)

          {acc + paths_from_next, new_cache}
      end

    new_cache = Map.put(updated_cache, {machine, stops}, paths)
    {paths, new_cache}
  end
end
