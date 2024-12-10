defmodule Part2 do
  def parse_line(line) do
    line
    |> String.split(~r/\s+/, trim: true)
    |> then(fn [left_str, right_str] ->
      {String.to_integer(left_str), String.to_integer(right_str)}
    end)
  end

  def parse_input(input) do
    {lefts, rights} =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(&parse_line/1)
      |> Enum.unzip()

    {Enum.sort(lefts), Enum.sort(rights)}
  end

  def part2(input) do
    {left_list, right_list} = input |> parse_input()

    right_map = Enum.frequencies(right_list)

    Enum.reduce(left_list, 0, fn left, acc ->
      case Map.get(right_map, left) do
        nil -> acc
        count -> acc + left * count
      end
    end)
  end
end

File.read!("input.txt")
|> Part2.part2()
|> IO.inspect(label: "part2")
