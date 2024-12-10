# frozen_string_literal: true

DIAGONAL_DIRECTIONS = [
  [1, 1], [-1, -1],  # First diagonal
  [1, -1], [-1, 1]   # Second diagonal
].freeze

def valid_position?(row, col, grid)
  row >= 0 && row < grid.length && col >= 0 && col < grid[0].length
end

def chars_at_pattern?(row, col, direction, grid)
  positions = (-1..1).map do |offset|
    [row + (offset * direction[0]), col + (offset * direction[1])]
  end

  return [] unless positions.all? { |pos_row, pos_col| valid_position?(pos_row, pos_col, grid) }

  positions.map { |pos_row, pos_col| grid[pos_row][pos_col] }
end

def pattern_at_position?(row, col, direction, grid)
  chars = chars_at_pattern?(row, col, direction, grid)
  return false if chars.empty?

  [%w[M A S], %w[S A M]].include?(chars)
end

def x_pattern_at?(row, col, grid)
  return false unless grid[row][col] == 'A'

  DIAGONAL_DIRECTIONS.each_slice(2).any? do |dir1, dir2|
    pattern_at_position?(row, col, dir1, grid) &&
      pattern_at_position?(row, col, dir2, grid)
  end
end

def count_x_mas(grid)
  rows = grid.length
  cols = grid[0].length

  (1..rows - 2).sum do |row|
    (1..cols - 2).count do |col|
      x_pattern_at?(row, col, grid)
    end
  end
end

def read_input(input)
  input.split("\n").map(&:chars)
end

def part2(input)
  grid = read_input(input)
  count_x_mas(grid)
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts "Result: #{part2(input)}"
end
