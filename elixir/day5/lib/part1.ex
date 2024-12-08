defmodule Part1 do
  defmodule Graph do
    defstruct edges: %{}, positions_cache: %{}

    def new(rules) do
      edges =
        Enum.reduce(rules, %{}, fn {from, to}, acc ->
          Map.update(acc, from, MapSet.new([to]), &MapSet.put(&1, to))
        end)

      %Graph{edges: edges, positions_cache: %{}}
    end

    def is_valid_sequence(%Graph{} = graph, sequence) do
      positions_cache =
        sequence
        |> Enum.with_index()
        |> Map.new()

      graph = %{graph | positions_cache: positions_cache}

      Enum.all?(sequence, &valid_position?(graph, &1))
    end

    defp valid_position?(graph, page) do
      case Map.get(graph.edges, page) do
        nil -> true
        deps -> Enum.all?(deps, &valid_dependency?(graph, page, &1))
      end
    end

    defp valid_dependency?(graph, page, dep) do
      case Map.get(graph.positions_cache, dep) do
        nil -> true
        dep_pos -> Map.get(graph.positions_cache, page) <= dep_pos
      end
    end
  end

  def find_middle(sequence), do: Enum.at(sequence, div(length(sequence), 2))

  def parse_input(input) do
    [rules_section, updates_section] = String.split(input, "\n\n", trim: true)

    rules =
      rules_section
      |> String.split("\n", trim: true)
      |> Enum.map(&parse_rule/1)

    updates =
      updates_section
      |> String.split("\n", trim: true)
      |> Enum.map(&parse_update/1)

    {rules, updates}
  end

  defp parse_rule(line) do
    [from, to] = String.split(line, "|")
    {String.trim(from) |> String.to_integer(), String.trim(to) |> String.to_integer()}
  end

  defp parse_update(line) do
    line
    |> String.split(",")
    |> Enum.map(&(String.trim(&1) |> String.to_integer()))
  end

  def part1(input) do
    {rules, updates} = parse_input(input)
    graph = Graph.new(rules)

    updates
    |> Enum.filter(&Graph.is_valid_sequence(graph, &1))
    |> Enum.map(&find_middle/1)
    |> Enum.sum()
  end
end

File.read!(~c"input.txt")
|> Part1.part1()
|> IO.inspect(label: "part2")
