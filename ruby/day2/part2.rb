# frozen_string_literal: true

def parse_input(input)
  input.split("\n").map { |line| line.split.map(&:to_i) }
end

def count_safe_reports(reports)
  reports.select { safe?(_1) }.count
end

def safe_sequence?(report)
  return false if report.length < 2

  diffs = report.each_cons(2).map { |a, b| b - a }
  return false unless diffs.all? { |d| d.abs.between?(1, 3) }

  diffs.all?(&:positive?) || diffs.all?(&:negative?)
end

def safe?(report)
  return true if safe_sequence?(report)

  report.length.times do |i|
    modified = report.dup
    modified.delete_at(i)
    return true if safe_sequence?(modified)
  end

  false
end

def part2(input)
  reports = parse_input(input)
  count_safe_reports(reports)
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  p part2(input)
end
