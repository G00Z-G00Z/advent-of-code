import dotenv
import re
import os
from pathlib import Path

dotenv.load_dotenv()

is_debug = True if os.environ.get("DEMO_APP", "1") == "1" else False
FILE_PATH = Path("./demo-input.txt") if is_debug else Path("./input.txt")

regex = r"one|two|three|four|five|six|seven|eight|nine|[0-9]"


def read_file(file_path: str) -> str:
    with open(file_path, "r") as f:
        return f.read()


numer_values = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

number_names = set(numer_values.keys())


def sol_2(content: str) -> int:
    total = 0
    for line in content.split("\n"):
        if not line:
            continue

        print(f"Line: {line}")
        matches = re.findall(regex, line)
        # First match
        first_digit = str(matches[0])

        # Second match
        second_digit = str(matches[-1])
        print(f"First digit: {first_digit}")
        print(f"Second digit: {second_digit}")

        first_value = numer_values.get(first_digit) or int(first_digit)
        second_value = numer_values.get(second_digit) or int(second_digit)
        value = first_value * 10 + second_value
        print(f"Total: {value}")
        total += value

    return total


def main():
    content = read_file(str(FILE_PATH))
    sum = sol_2(content)
    print(f"Sum: {sum}")


if __name__ == "__main__":
    main()
    pass
