# frozen_string_literal: true

class Node
  attr_reader :value, :left, :right, :result

  def initialize(result:, value:, left:, right:)
    @result = result
    @value = value
    @left = left
    @right = right
  end

  def evaluate(current_value = @value)
    return current_value == @result if @left.nil? && @right.nil?

    return true if current_value == @result

    [@left, @right].compact.each do |child|
      # Try addition
      return true if child.evaluate(current_value + child.value)
      # Try multiplication
      return true if child.evaluate(current_value * child.value)
    end

    false
  end

  def self.build_tree(result, numbers)
    return nil if numbers.empty?
    return Node.new(result: result, value: numbers.first, left: nil, right: nil) if numbers.size == 1

    first = numbers.first
    rest = numbers[1..]

    Node.new(
      result: result,
      value: first,
      left: build_tree(result, rest),
      right: build_tree(result, rest)
    )
  end
end

def parse_input(input)
  input.lines.map do |line|
    result, numbers = line.strip.split(': ')
    [result.to_i, numbers.split.map(&:to_i)]
  end
end

def evaluate_line(result, numbers)
  tree = Node.build_tree(result, numbers)
  tree.evaluate ? result : 0
end

def part1(input)
  lines = parse_input(input)
  lines.sum { |result, numbers| evaluate_line(result, numbers) }
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts part1(input)
end
