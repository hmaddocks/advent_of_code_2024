defmodule Part2 do
  @valid_patterns MapSet.new([
                    {["M", "S"], ["M", "S"]},
                    {["M", "S"], ["S", "M"]},
                    {["S", "M"], ["M", "S"]},
                    {["S", "M"], ["S", "M"]}
                  ])

  def count_x_mas_patterns(grid) do
    cols = length(Enum.at(grid, 0))

    grid
    |> Enum.chunk_every(3, 1, :discard)
    |> Enum.with_index()
    |> Enum.reduce(0, fn {window, _row}, acc ->
      acc + count_patterns_in_window(window, cols)
    end)
  end

  defp count_patterns_in_window([top, middle, bottom], cols) do
    0..(cols - 3)
    |> Enum.reduce(0, fn col, acc ->
      pattern = extract_pattern(top, middle, bottom, col)
      if is_x_mas_pattern?(pattern), do: acc + 1, else: acc
    end)
  end

  defp extract_pattern(top, middle, bottom, col) do
    {
      Enum.at(top, col),
      Enum.at(top, col + 2),
      Enum.at(middle, col + 1),
      Enum.at(bottom, col),
      Enum.at(bottom, col + 2)
    }
  end

  defp is_x_mas_pattern?({_, _, "A", _, _} = pattern), do: valid_mas_combinations?(pattern)
  defp is_x_mas_pattern?(_), do: false

  defp valid_mas_combinations?({tl, tr, _center, bl, br}) do
    diagonal1 = [tl, br]
    diagonal2 = [tr, bl]

    MapSet.member?(@valid_patterns, {diagonal1, diagonal2}) or
      MapSet.member?(@valid_patterns, {diagonal2, diagonal1})
  end

  def read_input(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&String.graphemes/1)
  end

  def part2(input) do
    input
    |> read_input()
    |> count_x_mas_patterns()
  end
end

File.read!(~c"input.txt")
|> Part2.part2()
|> IO.inspect(label: "part2")
