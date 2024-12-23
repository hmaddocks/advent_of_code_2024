# frozen_string_literal: true

require_relative 'part2'

RSpec.describe 'Part2' do
  it 'adds two numbers' do
    input = <<~INPUT
      7 6 4 2 1
      1 2 7 8 9
      9 7 6 2 1
      1 3 2 4 5
      8 6 4 4 1
      1 3 6 7 9
    INPUT
    expect(part2(input)).to eq(4)
  end
end
