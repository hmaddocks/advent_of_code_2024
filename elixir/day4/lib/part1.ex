defmodule Part1 do
  @directions [
    {0, 1},
    {1, 0},
    {0, -1},
    {-1, 0},
    {1, 1},
    {1, -1},
    {-1, 1},
    {-1, -1}
  ]

  def find_word(grid, word) when is_list(grid) and is_binary(word) do
    word
    |> String.graphemes()
    |> find_word_in_grid(grid)
  end

  defp find_word_in_grid(word_chars, grid) do
    dimensions = {length(grid), length(hd(grid))}

    for row <- 0..(elem(dimensions, 0) - 1),
        col <- 0..(elem(dimensions, 1) - 1),
        direction <- @directions,
        match_at_position?(grid, {row, col}, direction, word_chars, dimensions),
        reduce: 0 do
      acc -> acc + 1
    end
  end

  defp match_at_position?(grid, {row, col}, {row_dir, col_dir}, chars, {rows, cols}) do
    chars
    |> Enum.with_index()
    |> Enum.all?(fn {char, i} ->
      pos = {row + i * row_dir, col + i * col_dir}
      in_bounds?(pos, {rows, cols}) && at(grid, pos) == char
    end)
  end

  defp in_bounds?({row, col}, {rows, cols}) do
    row >= 0 and row < rows and col >= 0 and col < cols
  end

  defp at(grid, {row, col}) do
    grid |> Enum.at(row) |> Enum.at(col)
  end

  defp read_input(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&String.graphemes/1)
  end

  def part1(input) when is_binary(input) do
    input
    |> read_input()
    |> find_word("XMAS")
  end
end

File.read!(~c"input.txt")
|> Part1.part1()
|> IO.inspect(label: "part1")
