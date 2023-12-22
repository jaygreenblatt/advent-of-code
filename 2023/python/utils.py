"""
Advent of Code 2020: Utils

https://github.com/norvig/pytudes/blob/main/ipynb/AdventUtils.ipynb
"""


def get_lines(filename: str) -> list[str]:
    """
    Get the text from a file.
    """
    with open(f"inputs/{filename}", "r") as file:
        return file.readlines()
