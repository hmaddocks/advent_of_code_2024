# frozen_string_literal: true

DIRECTIONS = [[-1, 0], [1, 0], [0, -1], [0, 1]].freeze

Point = Struct.new(:row, :col) do
  def neighbors(rows:, cols:)
    DIRECTIONS.filter_map do |(dx, dy)|
      new_row = row + dx
      new_col = col + dy
      next unless (0...rows).cover?(new_row) && (0...cols).cover?(new_col)

      Point.new(new_row, new_col)
    end
  end
end

def parse_input(input)
  input.each_line(chomp: true).map { |line| line.chars.map(&:to_i) }
end

def find_starting_points(grid)
  Enumerator.new do |yielder|
    grid.each_with_index do |row, r|
      row.each_with_index.select { |val, _| val.zero? }
         .each { |_, c| yielder << Point.new(r, c) }
    end
  end
end

def valid_next_point?(grid:, point:, target_value:, visited:)
  grid[point.row][point.col] == target_value && !visited.include?(point)
end

def process_path(grid:, path:, visited:, queue:)
  current = path.last
  current_value = grid[current.row][current.col]
  return 1 if current_value == 9

  target_value = current_value + 1
  process_neighbors(
    grid: grid,
    path: path,
    current: current,
    target_value: target_value,
    visited: visited,
    queue: queue
  )
  0
end

def process_neighbors(grid:, path:, current:, target_value:, visited:, queue:)
  current.neighbors(rows: grid.length, cols: grid[0].length).each do |next_point|
    next unless valid_next_point?(
      grid: grid,
      point: next_point,
      target_value: target_value,
      visited: visited
    )

    visited.add(next_point)
    new_path = path.dup << next_point
    queue.push(new_path)
  end
end

def find_paths(grid)
  path_count = 0
  search_state = { queue: [], visited: Set.new }

  find_starting_points(grid).each do |start|
    search_state[:queue].clear
    search_state[:visited].clear

    search_state[:visited].add(start)
    search_state[:queue].push([start])

    while (path = search_state[:queue].shift)
      path_count += process_path(
        grid: grid,
        path: path,
        visited: search_state[:visited],
        queue: search_state[:queue]
      )
    end
  end

  path_count
end

def part1(input)
  grid = parse_input(input)
  find_paths(grid)
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts part1(input)
end
