defmodule AocLib.Types do
  @moduledoc """
  Types for Advent of Code problem inputs and results.
  """

  defmodule ProblemInput do
    @moduledoc """
    Represents the input data for an Advent of Code problem.
    """
    defstruct [:lines, :filename]

    @type t :: %__MODULE__{
            lines: [String.t()],
            filename: String.t()
          }

    @doc """
    Reads input from a file.
    """
    @spec read(String.t()) :: {:ok, t()} | {:error, atom()}
    def read(filename) do
      case File.read(filename) do
        {:ok, content} ->
          lines =
            content
            |> String.split("\n")
            |> Enum.map(&String.trim_trailing(&1, "\r"))

          {:ok, %__MODULE__{lines: lines, filename: filename}}

        {:error, reason} ->
          {:error, reason}
      end
    end

    @doc """
    Returns all lines as a list of strings.
    """
    @spec lines(t()) :: [String.t()]
    def lines(%__MODULE__{lines: lines}), do: lines

    @doc """
    Returns the input as a single string (all lines joined).
    """
    @spec string(t()) :: String.t()
    def string(%__MODULE__{lines: lines}), do: Enum.join(lines, "")

    @doc """
    Returns the input as a 2D grid (list of lists of characters).
    """
    @spec grid(t()) :: [[String.t()]]
    def grid(%__MODULE__{lines: lines}) do
      lines
      |> Enum.map(&String.graphemes/1)
    end
  end

  defmodule ProblemResult do
    @moduledoc """
    Represents the result of solving an Advent of Code problem.
    Can be a special atom, an integer, or a string.
    """

    @type t ::
            :no_input
            | :no_sample
            | :unsolved
            | :no_part_two
            | integer()
            | String.t()

    @doc """
    Formats a result for display.
    """
    @spec format(t()) :: String.t()
    def format(:no_input), do: "<No Input Available>"
    def format(:no_sample), do: "<No Sample Defined>"
    def format(:unsolved), do: "<No Solution Implemented>"
    def format(:no_part_two), do: "<No Part Two>"
    def format(value) when is_integer(value), do: Integer.to_string(value)
    def format(value) when is_binary(value), do: value
    def format(value), do: inspect(value)

    @doc """
    Checks if two results are equal.
    """
    @spec equal?(t(), t()) :: boolean()
    def equal?(result1, result2), do: result1 == result2
  end
end
