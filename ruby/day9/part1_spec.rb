# frozen_string_literal: true

require_relative 'part1'

RSpec.describe 'Part1' do
  describe '#initialize_disk' do
    it 'creates disk array from sequence' do
      sequence = [2, 1, 1, 2]
      expect(initialize_disk(sequence)).to eq([0, 0, nil, 1, nil, nil])
    end

    it 'handles sequence without spaces' do
      sequence = [2]
      expect(initialize_disk(sequence)).to eq([0, 0])
    end
  end

  describe '#find_empty_slots' do
    it 'identifies empty slots in disk' do
      disk = [0, nil, 1, nil, 2]
      expect(find_empty_slots(disk)).to eq(Set.new([1, 3]))
    end

    it 'returns empty set when no empty slots' do
      disk = [0, 1, 2]
      expect(find_empty_slots(disk)).to eq(Set.new)
    end
  end

  describe '#move_files_left' do
    it 'moves files to leftmost empty slots' do
      disk = [0, nil, 1, nil, 2]
      empty_slots = Set.new([1, 3])
      expect(move_files_left(disk, empty_slots)).to eq([0, 2, 1, nil, nil])
    end

    it 'handles disk with no empty slots' do
      disk = [0, 1, 2]
      empty_slots = Set.new
      expect(move_files_left(disk, empty_slots)).to eq([0, 1, 2])
    end
  end

  describe '#calculate_score' do
    it 'calculates score based on file positions' do
      disk = [0, 1, nil, 2]
      expect(calculate_score(disk)).to eq(7) # 0*0 + 1*1 + 0 + 3*2 = 7
    end

    it 'returns 0 for empty disk' do
      expect(calculate_score([])).to eq(0)
    end
  end

  describe '#optimize_disk_space' do
    it 'optimizes disk space and calculates score' do
      input = [2, 1, 1, 2]
      expect(optimize_disk_space(input)).to eq(2)
    end
  end

  it 'solves part1' do
    input = '2333133121414131402'
    expect(part1(input)).to eq 1928
  end
end
