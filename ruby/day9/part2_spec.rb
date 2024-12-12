# frozen_string_literal: true

require_relative 'part2'

RSpec.describe 'Part2' do
  it 'adds two numbers' do
    input = '2333133121414131402'
    expect(part2(input)).to eq 2858
  end
end
