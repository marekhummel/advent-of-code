defmodule AocLib.Util do
  @moduledoc """
  Utility functions for Advent of Code solutions.
  """

  @doc """
  Counts the number of digits in a non-negative integer.
  """
  @spec count_digits(non_neg_integer()) :: non_neg_integer()
  def count_digits(n) when n >= 0 do
    ceil(:math.log10(n + 1))
  end

  @doc """
  Computes 10 raised to the power of `exp`.
  """
  @spec power_of_10(non_neg_integer()) :: non_neg_integer()
  def power_of_10(exp), do: trunc(:math.pow(10, exp))

  @doc """
  Converts a boolean to an integer (1 for true, 0 for false).
  """
  @spec bool_to_int(boolean()) :: 0 | 1
  def bool_to_int(true), do: 1
  def bool_to_int(false), do: 0

  @doc """
  Transposes a grid (list of lists).
  """
  @spec grid_transpose([[any()]], any()) :: [[any()]]
  def grid_transpose(grid, fill \\ nil) do
    grid |> zip_longest(fill) |> Enum.map(&Tuple.to_list/1)
  end

  @doc """
  Zips multiple enumerables together, filling shorter ones with a specified value.
  """
  @spec zip_longest([Enumerable.t()], any()) :: [tuple()]
  def zip_longest(enumerables, fill \\ nil) do
    max_length = enumerables |> Enum.map(&Enum.count/1) |> Enum.max()

    enumerables
    |> Enum.map(fn enum ->
      enum
      |> Stream.concat(Stream.cycle([fill]))
      |> Enum.take(max_length)
    end)
    |> Enum.zip()
  end

  @doc """
  Generates all combinations of `k` elements from the given list.
  """
  @spec combinations(non_neg_integer(), [any()]) :: [[any()]]
  def combinations(_elements, 0), do: [[]]
  def combinations([], _k), do: []

  def combinations([h | t], k),
    do: for(comb <- combinations(t, k - 1), do: [h | comb]) ++ combinations(t, k)

  defmodule Vec3 do
    defstruct [:x, :y, :z]

    @type t :: %__MODULE__{
            x: number(),
            y: number(),
            z: number()
          }

    @doc """
    Creates a new 3D vector.
    """
    @spec new(number(), number(), number()) :: t()
    def new(x, y, z), do: %__MODULE__{x: x, y: y, z: z}

    @doc """
    Computes the squared eucledian distance between two 3D vectors.
    """
    @spec dist_sq(t(), t()) :: number()
    def dist_sq(%__MODULE__{x: x1, y: y1, z: z1}, %__MODULE__{x: x2, y: y2, z: z2}) do
      dx = x2 - x1
      dy = y2 - y1
      dz = z2 - z1
      dx * dx + dy * dy + dz * dz
    end
  end

  defmodule UnionFind do
    defstruct parents: %{}

    @type t :: %__MODULE__{
            parents: %{any() => any()}
          }

    @doc """
    Creates a new Union-Find structure with the given elements.
    """
    @spec new([any()]) :: t()
    def new(elements), do: %__MODULE__{parents: Map.new(elements, fn e -> {e, e} end)}

    @doc """
    Finds the root of the set containing the given element, with path compression.
    """
    @spec find(t(), any()) :: {any(), t()}
    def find(%__MODULE__{parents: parents} = uf, element) do
      case Map.get(parents, element) do
        ^element ->
          {element, uf}

        parent_element ->
          {root, uf} = find(uf, parent_element)
          updated_parents = Map.put(uf.parents, element, root)
          {root, %__MODULE__{parents: updated_parents}}
      end
    end

    @doc """
    Unites the sets containing the two given elements.
    """
    @spec union(t(), any(), any()) :: t()
    def union(uf, element1, element2) do
      {root1, uf} = find(uf, element1)
      {root2, uf} = find(uf, element2)

      if root1 != root2 do
        updated_parents = Map.put(uf.parents, root2, root1)
        %__MODULE__{parents: updated_parents}
      else
        uf
      end
    end

    @doc """
    Returns a map of roots to the size of their respective sets.
    """
    @spec sets(t()) :: {map(), t()}
    def sets(%__MODULE__{parents: parents} = uf) do
      {roots, uf} =
        parents |> Map.keys() |> Enum.map_reduce(uf, fn e, acc_uf -> find(acc_uf, e) end)

      {roots |> Enum.frequencies(), uf}
    end
  end
end
