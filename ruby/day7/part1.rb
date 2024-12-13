# frozen_string_literal: true

class Node # rubocop:disable Style/Documentation
  attr_reader :result, :value, :left, :right

  def initialize(result:, value:, left: nil, right: nil)
    @result = result
    @value = value
    @left = left
    @right = right
  end

  def evaluate(current_value = value)
    # Short circuit impossible cases early
    return false if current_value > result
    return true if current_value == result
    return false if leaf?

    children.any? do |child|
      evaluate_operation(current_value, child)
    end
  end

  def self.build_tree(result, numbers)
    return nil if numbers.empty?
    return new(result:, value: numbers.first) if numbers.one?

    first, *rest = numbers
    new(
      result:,
      value: first,
      left: build_tree(result, rest),
      right: build_tree(result, rest)
    )
  end

  private

  def leaf?
    left.nil? && right.nil?
  end

  def children
    [left, right].compact
  end

  def evaluate_operation(current, child)
    child.evaluate(current + child.value) ||
      child.evaluate(current * child.value)
  end
end

def evaluate_line(result, numbers)
  tree = Node.build_tree(result, numbers)
  tree.evaluate ? result : 0
end

def parse_input(input)
  input.each_line.map do |line|
    result, numbers = line.strip.split(': ')
    [result.to_i, numbers.split.map(&:to_i)]
  end
end

def part1(input)
  lines = parse_input(input)
  lines.sum { |result, numbers| evaluate_line(result, numbers) }
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts part1(input)
end
