# frozen_string_literal: true

require_relative 'part1'

RSpec.describe 'Part1' do
  it 'works' do
    input = '0 1 10 99 999'
    expect(part1(input, 1)).to eq 7
  end

  it 'works part 2' do
    input = '125 17'
    expect(part1(input, 6)).to eq 22
  end

  it 'works part 3' do
    input = '8435 234 928434 14 0 7 92446 8992692'
    expect(part1(input, 25)).to eq 182_081
  end
end
