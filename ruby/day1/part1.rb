# frozen_string_literal: true

def parse_input(input)
  left, right = input.split("\n")
                     .map { |line| line.split.map(&:to_i) }
                     .transpose
  [left.sort, right.sort]
end

def part1(input)
  left, right = parse_input(input)
  left.zip(right)
      .sum { |l, r| (l - r).abs }
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  p part1(input)
end
