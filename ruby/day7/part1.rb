# frozen_string_literal: true

class Node # rubocop:disable Style/Documentation
  attr_reader :target, :value, :left, :right

  def initialize(target:, value:, left: nil, right: nil)
    @target = target
    @value = value
    @left = left
    @right = right
  end

  def evaluate(current = value)
    return current == target if leaf?
    return true if current == target

    # Short circuit only if we're definitely too high
    return false if current > target

    children.any? do |child|
      child.evaluate(current + child.value) ||
        child.evaluate(current * child.value)
    end
  end

  def self.build_tree(target, numbers)
    return nil if numbers.empty?
    return new(target: target, value: numbers.first) if numbers.size == 1

    first, *rest = numbers
    new(
      target: target,
      value: first,
      left: build_tree(target, rest),
      right: build_tree(target, rest)
    )
  end

  private

  def leaf?
    left.nil? && right.nil?
  end

  def children
    [left, right].compact
  end
end

def evaluate_line(target, numbers)
  # Short circuit if sum would exceed target or product would be too small
  return 0 if numbers.sum > target
  return 0 if numbers.reduce(1, :*) < target

  tree = Node.build_tree(target, numbers)
  tree.evaluate ? target : 0
end

def parse_input(input)
  input.each_line.map do |line|
    target, numbers = line.strip.split(': ')
    [target.to_i, numbers.split.map(&:to_i)]
  end
end

def part1(input)
  lines = parse_input(input)
  lines.sum { |target, numbers| evaluate_line(target, numbers) }
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts part1(input)
end
