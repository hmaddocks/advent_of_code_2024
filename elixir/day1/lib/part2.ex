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
    input |> parse_input()
  end
end

File.read!("input.txt")
|> Part2.part2()
|> IO.inspect(label: "part2")
