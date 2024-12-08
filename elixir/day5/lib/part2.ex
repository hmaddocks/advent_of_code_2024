defmodule Part2 do
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
    input = String.trim(input)
    [rules_section, updates_section] = String.split(input, "\n\n", trim: true)

    rules =
      rules_section
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        [from, to] = String.split(line, "|")
        {String.trim(from) |> String.to_integer(), String.trim(to) |> String.to_integer()}
      end)

    updates =
      updates_section
      |> String.split("\n", trim: true)
      |> Enum.map(&parse_update/1)
      # Filter out any non-list values
      |> Enum.filter(&is_list/1)

    {rules, updates}
  end

  defp parse_update(line) do
    line
    |> String.split(",")
    |> Enum.map(&(String.trim(&1) |> String.to_integer()))
  end

  def is_correctly_ordered?(rules, sequence) do
    graph = Graph.new(rules)
    Graph.is_valid_sequence(graph, sequence)
  end

  def should_come_before?(rules, num1, num2) do
    graph = Graph.new(rules)

    case Map.get(graph.edges, num1) do
      nil ->
        false

      deps ->
        deps |> MapSet.member?(num2) or Enum.any?(deps, &should_come_before?(rules, &1, num2))
    end
  end

  def reorder_sequence(rules, sequence) do
    sequence
    |> Enum.sort(fn num1, num2 ->
      cond do
        should_come_before?(rules, num1, num2) -> true
        should_come_before?(rules, num2, num1) -> false
        true -> num1 > num2
      end
    end)
  end

  def part2(input) do
    {rules, updates} = parse_input(input)

    updates
    |> Enum.reject(&is_correctly_ordered?(rules, &1))
    |> Enum.map(fn seq -> reorder_sequence(rules, seq) end)
    |> Enum.map(&find_middle/1)
    |> Enum.sum()
  end
end

File.read!(~c"input.txt")
|> Part2.part2()
|> IO.inspect(label: "part2")
