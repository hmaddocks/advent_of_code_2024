# frozen_string_literal: true

class Region
  attr_accessor :area, :perimeter

  def initialize
    @area = 0
    @perimeter = 0
  end

  def score
    area * perimeter
  end
end

DIRECTIONS = [[-1, 0], [1, 0], [0, -1], [0, 1]].freeze

def parse_input(input)
  input.strip.lines.map { |line| line.strip.chars }
end

def flood_fill(grid, visited, start_i, start_j, ch)
  region = Region.new
  rows = grid.length
  cols = grid[0].length
  stack = [[start_i, start_j]]

  while (i, j = stack.pop)
    next if visited[i][j] || grid[i][j] != ch

    visited[i][j] = true
    region.area += 1

    # Count boundary edges
    region.perimeter += 1 if i == 0
    region.perimeter += 1 if i == grid.length - 1
    region.perimeter += 1 if j == 0
    region.perimeter += 1 if j == grid[0].length - 1

    # Check all adjacent cells
    DIRECTIONS.each do |di, dj|
      ni = i + di
      nj = j + dj

      if ni >= 0 && ni < rows && nj >= 0 && nj < cols
        if grid[ni][nj] != ch
          region.perimeter += 1
        elsif !visited[ni][nj]
          stack.push([ni, nj])
        end
      end
    end
  end

  region
end

def find_regions(grid)
  rows = grid.length
  cols = grid[0].length
  visited = Array.new(rows) { Array.new(cols, false) }
  regions = []

  (0...rows).each do |i|
    (0...cols).each do |j|
      regions << flood_fill(grid, visited, i, j, grid[i][j]) unless visited[i][j]
    end
  end

  regions
end

def part1(input)
  grid = parse_input(input)
  find_regions(grid).sum(&:score)
end

# Main program
if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts part1(input)
end
