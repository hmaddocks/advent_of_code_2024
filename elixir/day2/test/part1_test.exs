defmodule Part1Test do
  use ExUnit.Case

  test "is_safe with various sequences" do
    # Too short sequences
    refute Part1.is_safe([1])
    refute Part1.is_safe([])

    # Valid increasing sequences
    # Minimal increasing
    assert Part1.is_safe([1, 2])
    # Max allowed increase
    assert Part1.is_safe([1, 3])
    # Longer increasing
    assert Part1.is_safe([1, 2, 4, 6])

    # Valid decreasing sequences
    # Minimal decreasing
    assert Part1.is_safe([6, 4])
    # Longer decreasing
    assert Part1.is_safe([10, 8, 6, 4])

    # Invalid sequences
    # Difference > 3
    refute Part1.is_safe([1, 5])
    # No difference
    refute Part1.is_safe([1, 1])
    # Mixed increasing/decreasing
    refute Part1.is_safe([1, 2, 1])
    # Mixed decreasing/increasing
    refute Part1.is_safe([3, 1, 4])

    # Edge cases
    # All differences = 1
    assert Part1.is_safe([1, 2, 3])
    # All differences = 3
    assert Part1.is_safe([1, 4, 7])
    # All differences = -3
    assert Part1.is_safe([7, 4, 1])
  end

  test "part1" do
    input = """
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    """

    assert Part1.part1(input) == 2
  end
end
