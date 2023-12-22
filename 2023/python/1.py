"""
Solution to Day 1.
"""
from utils import get_lines

FILENAME = "1.txt"


def get_digits(line: str) -> list[int]:
    """
    Get a list of all digits in the line.
    """
    return [int(d) for d in line if d.isdigit()]


def part_one():
    """
    Solution to Part 1.
    """
    lines = get_lines(FILENAME)
    answer = 0

    for line in lines:
        digits = get_digits(line)
        value = int(digits[0]) * 10 + int(digits[len(digits) - 1])
        answer += value

    return answer


if __name__ == "__main__":
    print(part_one())
