# frozen_string_literal: true

require_relative 'part1'

RSpec.describe 'Part1' do
  it 'adds two numbers' do
    input = <<~INPUT
      MMMSXXMASM
      MSAMXMSMSA
      AMXSXMAAMM
      MSAMASMSMX
      XMASAMXAMM
      XXAMMXXAMA
      SMSMSASXSS
      SAXAMASAAA
      MAMMMXMMMM
      MXMXAXMASX
    INPUT

    expect(part1(input)).to eq 18
  end
end
