defmodule AdventOfCode.MixProject do
  use Mix.Project

  # Years share module names, so each year has its own compilation directory.
  def project do
    year =
      case System.argv() do
        ["aoc", year | _] -> year
        _ -> "2025"
      end

    [
      app: :advent_of_code,
      version: "1.0.0",
      elixirc_paths: ["src/lib/elixir", "src/#{year}/elixir"],
      build_path: "build/elixir/#{year}",
      deps_path: "build/elixir/deps",
      dialyzer: [plt_core_path: "build/elixir/plts", plt_local_path: "build/elixir/plts"],
      deps: [{:dialyxir, "~> 1.4", only: :dev, runtime: false}],
      aliases: [aoc: &aoc/1]
    ]
  end

  # mix aoc YEAR builds; mix aoc YEAR DAY PART MODE builds and runs.
  defp aoc([_year | args]) do
    if args == [] do
      Mix.Task.run("compile")
    else
      Mix.Task.run("run", ["--no-start", "-e", "Main.main(System.argv())", "--" | args])
    end
  end
end
