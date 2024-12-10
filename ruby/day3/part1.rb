# frozen_string_literal: true

def part1(input)
  input.scan(/mul\((\d+),(\d+)\)/).sum { |a, b| a.to_i * b.to_i }
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  p part1(input)
end
