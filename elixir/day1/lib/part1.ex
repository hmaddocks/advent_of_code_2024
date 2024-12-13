defmodule Part1 do
  def parse_line(line) do
    line
    |> String.split(~r/\s+/, trim: true)
    |> Enum.map(fn [left_str, right_str] ->
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

  def part1(input) do
    {left_list, right_list} = parse_input(input)

    Enum.zip(left_list, right_list)
    |> Enum.map(fn {l, r} -> abs(l - r) end)
    |> Enum.sum()
  end
end

File.read!(~c"input.txt")
|> Part1.part1()
|> IO.inspect(label: "part1")
