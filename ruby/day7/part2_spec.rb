# frozen_string_literal: true

require_relative 'part2'

RSpec.describe 'Part2' do
  it 'two numbers OK' do
    input = '190: 10 19'
    expect(part2(input)).to eq 190
  end

  it 'two numbers not OK' do
    input = '83: 17 5'
    expect(part2(input)).to eq 0
  end

  it 'three numbers OK' do
    input = '3267: 81 40 27'
    expect(part2(input)).to eq 3267
  end

  it 'three numbers not OK' do
    input = '161011: 16 10 13'
    expect(part2(input)).to eq 0
  end

  it 'four numbers OK' do
    input = '292: 11 6 16 20'
    expect(part2(input)).to eq 292
  end

  it 'four numbers not OK' do
    input = '7290: 6 8 6 15'
    expect(part2(input)).to eq 0
  end

  it 'concat two numbers OK' do
    input = '156: 15 6'
    expect(part2(input)).to eq 156
  end

  it 'concat three numbers OK' do
    input = '192: 17 8 14'
    expect(part2(input)).to eq 192
  end

  it 'concat four numbers OK' do
    input = '7290: 6 8 6 15'
    expect(part2(input)).to eq 7290
  end

  it 'solve puzzle' do
    input = <<~INPUT
      190: 10 19
      3267: 81 40 27
      83: 17 5
      156: 15 6
      7290: 6 8 6 15
      161011: 16 10 13
      192: 17 8 14
      21037: 9 7 18 13
      292: 11 6 16 20
    INPUT
    expect(part2(input)).to eq 11_387
  end
end
