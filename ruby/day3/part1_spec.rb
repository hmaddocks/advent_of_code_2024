# frozen_string_literal: true

require_relative 'part1'

RSpec.describe 'Part1' do
  it 'adds two numbers' do
    input = 'xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))'
    expect(part1(input)).to eq(161)
  end
end
