import dotenv
import os
from pathlib import Path

dotenv.load_dotenv()

is_debug = True if os.environ.get("DEMO_APP", "1") == "1" else False
FILE_PATH = Path("./demo-input.txt") if is_debug else Path("./input.txt")


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

        # Find indexes for numbers
        print(line)

        indexes = {name: line.find(name) for name in number_names}

        list_of_indexes = [
            (name, index, numer_values.get(name))
            for name, index in indexes.items()
            if index != -1
        ]

        # Find number from the left
        first_idx = -1
        value = -1
        for idx, char in enumerate(line):
            if char.isdigit():
                first_idx = idx
                value = int(char)
                break

        # Add indexes to list
        if first_idx != -1:
            list_of_indexes.append(("first", first_idx, value))

        # Find number from the right
        last_idx = -1
        value = -1
        for idx, char in enumerate(line[::-1]):
            if char.isdigit():
                last_idx = len(line) - idx - 1
                value = int(char)
                break

        if last_idx != -1:
            list_of_indexes.append(("last", last_idx, value))

        list_of_indexes.sort(key=lambda x: x[1])
        print(list_of_indexes)

        # Join the numbers
        first_no = list_of_indexes[0][2]
        second_no = list_of_indexes[-1][2]
        print(f"Found numbers {first_no} and {second_no}")
        total += first_no * 10 + second_no

    return total


def main():
    content = read_file(str(FILE_PATH))
    sum = sol_2(content)
    print(f"Sum: {sum}")


if __name__ == "__main__":
    main()
    pass
