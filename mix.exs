defmodule AdventOfCode.MixProject do
  use Mix.Project

  # Project definition for ElixirLS and checks; Task builds each year independently.
  def project do
    [
      app: :advent_of_code,
      version: "1.0.0",
      elixirc_paths: ["src/lib/elixir", "src/2025/elixir"],
      build_path: "build/elixir/mix",
      deps_path: "build/elixir/deps",
      dialyzer: [plt_core_path: "build/elixir/plts", plt_local_path: "build/elixir/plts"],
      deps: [{:dialyxir, "~> 1.4", only: :dev, runtime: false}]
    ]
  end
end
