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

  def count_occurrences(grid, word) do
    word_chars = String.graphemes(word)
    word_len = length(word_chars)
    rows = length(grid)
    cols = length(Enum.at(grid, 0))

    for row <- 0..(rows - 1),
        col <- 0..(cols - 1),
        direction <- @directions,
        reduce: 0 do
      count ->
        if check_word(grid, {row, col}, direction, word_chars, word_len, rows, cols) do
          count + 1
        else
          count
        end
    end
  end

  defp check_word(
         grid,
         {row, col},
         {row_direction, col_direction},
         word_chars,
         word_len,
         rows,
         cols
       ) do
    Enum.reduce_while(0..(word_len - 1), true, fn i, _acc ->
      nr = row + i * row_direction
      nc = col + i * col_direction

      cond do
        nr < 0 or nr >= rows or nc < 0 or nc >= cols ->
          {:halt, false}

        Enum.at(Enum.at(grid, nr), nc) != Enum.at(word_chars, i) ->
          {:halt, false}

        true ->
          {:cont, true}
      end
    end)
  end

  def read_input(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&String.graphemes/1)
  end

  def part1(input) do
    grid = read_input(input)
    count_occurrences(grid, "XMAS")
  end
end

File.read!(~c"input.txt")
|> Part1.part1()
|> IO.inspect(label: "part1")
