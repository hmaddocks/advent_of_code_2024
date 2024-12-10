# frozen_string_literal: true

require_relative 'part2'

RSpec.describe 'Part2' do
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

    expect(part2(input)).to eq 9
  end
end
