defmodule Part2 do
  def parse_input(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_line/1)
  end

  defp parse_line(line) do
    line
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end

  def is_safe([]), do: false
  def is_safe([_]), do: false
  def is_safe(levels) do
    levels
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.map(fn [a, b] -> b - a end)
    |> check_differences()
  end

  defp check_differences(diffs) do
    with false <- Enum.any?(diffs, &(abs(&1) < 1 or abs(&1) > 3)) do
      Enum.all?(diffs, &(&1 > 0)) or Enum.all?(diffs, &(&1 < 0))
    else
      true -> false
    end
  end

  def is_safe_after_removal(levels) do
    levels
    |> Enum.with_index()
    |> Enum.any?(fn {_, i} -> 
      levels
      |> List.delete_at(i)
      |> is_safe()
    end)
  end

  def count_safe_reports(reports) do
    Enum.count(reports, &is_safe_after_removal/1)
  end

  def part2(input) do
    input
    |> parse_input()
    |> count_safe_reports()
  end
end

File.read!(~c"input.txt")
|> Part2.part2()
|> IO.inspect(label: "part2")
