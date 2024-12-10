# frozen_string_literal: true

require_relative 'part2'

RSpec.describe 'Part1' do
  it 'adds two numbers' do
    input = <<~INPUT
      3   4
      4   3
      2   5
      1   3
      3   9
      3   3
    INPUT
    expect(part2(input)).to eq(31)
  end
end
