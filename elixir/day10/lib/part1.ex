defmodule Part1 do
  defmodule Point do
    defstruct [:row, :col]

    def new(row, col), do: %__MODULE__{row: row, col: col}

    def neighbors(%__MODULE__{row: row, col: col}, rows: rows, cols: cols) do
      [[-1, 0], [1, 0], [0, -1], [0, 1]]
      |> Enum.map(fn [dx, dy] -> {row + dx, col + dy} end)
      |> Enum.filter(fn {new_row, new_col} ->
        new_row in 0..(rows - 1) and new_col in 0..(cols - 1)
      end)
      |> Enum.map(fn {new_row, new_col} -> new(new_row, new_col) end)
    end
  end

  def parse_input(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      line
      |> String.graphemes()
      |> Enum.map(&String.to_integer/1)
    end)
  end

  def find_starting_points(grid) do
    for {row, r} <- Enum.with_index(grid),
        {val, c} <- Enum.with_index(row),
        val == 0,
        do: Point.new(r, c)
  end

  def valid_next_point?(grid, point, target_value, visited) do
    grid
    |> get_in([Access.at(point.row), Access.at(point.col)])
    |> Kernel.==(target_value)
    |> Kernel.and(point not in visited)
  end

  def process_path(grid, path, visited, queue) do
    current = List.last(path)
    current_value = get_in(grid, [Access.at(current.row), Access.at(current.col)])

    if current_value == 9 do
      1
    else
      target_value = current_value + 1
      process_neighbors(grid, path, current, target_value, visited, queue)
      0
    end
  end

  def process_neighbors(grid, path, current, target_value, visited, queue) do
    rows = length(grid)
    cols = length(hd(grid))

    current
    |> Point.neighbors(rows: rows, cols: cols)
    |> Enum.each(fn next_point ->
      if valid_next_point?(grid, next_point, target_value, visited) do
        new_visited = MapSet.put(visited, next_point)
        new_path = path ++ [next_point]
        :queue.in(new_path, queue)
      end
    end)
  end

  def find_paths(grid) do
    find_starting_points(grid)
    |> Enum.reduce(0, fn start, path_count ->
      queue = :queue.new()
      visited = MapSet.new([start])
      :queue.in([start], queue)

      search_paths(grid, queue, visited, path_count)
    end)
  end

  defp search_paths(grid, queue, visited, path_count) do
    case :queue.out(queue) do
      {{:value, path}, new_queue} ->
        new_count = path_count + process_path(grid, path, visited, new_queue)
        search_paths(grid, new_queue, visited, new_count)

      {:empty, _} ->
        path_count
    end
  end

  def part1(input) do
    input
    |> parse_input()
    |> find_paths()
  end
end

if System.argv() == ["--run"] do
  File.read!("../input.txt")
  |> Part1.part1()
  |> IO.inspect(label: "part1")
end
