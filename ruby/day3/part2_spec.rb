# frozen_string_literal: true

require_relative 'part2'

RSpec.describe 'Part2' do
  it 'adds two numbers' do
    input = 'xmul(2,4)&mul[3,7]!^don\'t()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))'
    expect(part2(input)).to eq(48)
  end
end
