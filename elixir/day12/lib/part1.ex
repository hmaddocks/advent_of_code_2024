defmodule Part1 do
  @directions [{-1, 0}, {1, 0}, {0, -1}, {0, 1}]

  defmodule Region do
    defstruct area: 0, perimeter: 0

    def score(%__MODULE__{area: area, perimeter: perimeter}) do
      area * perimeter
    end
  end

  def parse_input(input) do
    input
    |> String.trim()
    |> String.split("\n", trim: true)
    |> Enum.map(&(String.trim(&1) |> String.graphemes()))
  end

  def flood_fill(grid, visited, {start_i, start_j}, ch) do
    dims = {length(grid), length(Enum.at(grid, 0))}
    flood_fill_impl(grid, visited, [{start_i, start_j}], %Region{}, ch, dims)
  end

  defp flood_fill_impl(_, visited, [], region, _ch, _dims), do: {region, visited}

  defp flood_fill_impl(grid, visited, [{i, j} = point | rest], region, ch, dims = {_, _}) do
    cond do
      MapSet.member?(visited, point) or get_in(grid, [Access.at(i), Access.at(j)]) != ch ->
        flood_fill_impl(grid, visited, rest, region, ch, dims)

      true ->
        visited = MapSet.put(visited, point)
        region = update_region_stats(region, point, dims)

        {new_stack, additional_perimeter} =
          process_neighbors(grid, visited, point, ch, dims, rest)

        region = %{region | perimeter: region.perimeter + additional_perimeter}

        flood_fill_impl(grid, visited, new_stack, region, ch, dims)
    end
  end

  defp update_region_stats(region, {i, j}, {rows, cols}) do
    boundary_edges =
      [
        i == 0,
        i == rows - 1,
        j == 0,
        j == cols - 1
      ]
      |> Enum.count(& &1)

    %{region | area: region.area + 1, perimeter: region.perimeter + boundary_edges}
  end

  defp process_neighbors(grid, visited, {i, j}, ch, {rows, cols}, stack) do
    @directions
    |> Enum.reduce({stack, 0}, fn {di, dj}, {acc_stack, perim} ->
      ni = i + di
      nj = j + dj

      if valid_position?({ni, nj}, {rows, cols}) do
        neighbor_ch = get_in(grid, [Access.at(ni), Access.at(nj)])

        cond do
          neighbor_ch != ch -> {acc_stack, perim + 1}
          not MapSet.member?(visited, {ni, nj}) -> {[{ni, nj} | acc_stack], perim}
          true -> {acc_stack, perim}
        end
      else
        {acc_stack, perim}
      end
    end)
  end

  defp valid_position?({i, j}, {rows, cols}) do
    i >= 0 and i < rows and j >= 0 and j < cols
  end

  def find_regions(grid) do
    rows = length(grid)
    cols = length(Enum.at(grid, 0))
    visited = MapSet.new()

    for i <- 0..(rows - 1),
        j <- 0..(cols - 1),
        not MapSet.member?(visited, {i, j}),
        reduce: {[], visited} do
      {regions, visited} ->
        ch = get_in(grid, [Access.at(i), Access.at(j)])
        {region, new_visited} = flood_fill(grid, visited, {i, j}, ch)
        {[region | regions], new_visited}
    end
    |> elem(0)
  end

  def part1(input) do
    input
    |> parse_input()
    |> find_regions()
    |> Enum.map(&Region.score/1)
    |> Enum.sum()
  end
end

File.read!(~c"input.txt")
|> Part1.part1()
|> IO.inspect(label: "part1")
