# frozen_string_literal: true

DIRECTIONS = [
  [0, 1],    # Right
  [1, 0],    # Down
  [0, -1],   # Left
  [-1, 0],   # Up
  [1, 1],    # Down-Right
  [1, -1],   # Down-Left
  [-1, 1],   # Up-Right
  [-1, -1]   # Up-Left
].freeze

def parse_input(input)
  input.split("\n").map(&:chars)
end

def valid_position?(row, col, grid)
  row >= 0 && row < grid.length && col >= 0 && col < grid[0].length
end

def word_at_position?(row, col, direction, word, grid)
  word_chars = word.chars
  word_chars.each_with_index.all? do |char, i|
    new_row = row + (i * direction[0])
    new_col = col + (i * direction[1])
    valid_position?(new_row, new_col, grid) && grid[new_row][new_col] == char
  end
end

def count_xmas(grid, word)
  rows = grid.length
  cols = grid[0].length

  (0...rows).sum do |row|
    (0...cols).sum do |col|
      DIRECTIONS.count do |direction|
        word_at_position?(row, col, direction, word, grid)
      end
    end
  end
end

def part1(input)
  grid = parse_input(input)
  count_xmas(grid, 'XMAS')
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  p part1(input)
end
