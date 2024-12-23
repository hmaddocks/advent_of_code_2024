# lib/day7/part1.ex
defmodule Part2 do
  defmodule Node do
    defstruct [:result, :value, :left, :right]

    def new(result, value, left \\ nil, right \\ nil) do
      %__MODULE__{result: result, value: value, left: left, right: right}
    end

    def evaluate(%__MODULE__{} = node, current_value \\ nil) do
      value = current_value || node.value

      cond do
        value > node.result ->
          false

        value == node.result ->
          true

        leaf?(node) ->
          false

        true ->
          node
          |> children()
          |> Enum.any?(fn child -> evaluate_operation(value, child) end)
      end
    end

    def build_tree(_result, []), do: nil
    def build_tree(result, [number]), do: new(result, number)

    def build_tree(result, [first | rest]) do
      new(
        result,
        first,
        build_tree(result, rest),
        build_tree(result, rest)
      )
    end

    defp leaf?(%__MODULE__{left: nil, right: nil}), do: true
    defp leaf?(_node), do: false

    defp children(%__MODULE__{left: left, right: right}) do
      [left, right] |> Enum.reject(&is_nil/1)
    end

    defp evaluate_operation(current, child) do
      evaluate(child, current + child.value) ||
        evaluate(child, current * child.value)
    end
  end

  def evaluate_line(result, numbers) do
    tree = Node.build_tree(result, numbers)
    if Node.evaluate(tree), do: result, else: 0
  end

  def parse_input(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      [result, numbers] = String.split(line, ": ")

      {
        String.to_integer(result),
        numbers |> String.split() |> Enum.map(&String.to_integer/1)
      }
    end)
  end

  def part2(input) do
    input
    |> parse_input()
    |> Enum.map(fn {result, numbers} -> evaluate_line(result, numbers) end)
    |> Enum.sum()
  end
end

# Run if script is executed directly
if System.get_env("MIX_ENV") != "test" do
  File.read!(~c"input.txt")
  |> Part2.part2()
  |> IO.inspect(label: "part2")
end
