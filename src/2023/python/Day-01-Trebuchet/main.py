import dotenv
import os
from pathlib import Path

dotenv.load_dotenv()

is_debug = True if os.environ.get("DEMO_APP", "1") else False
FILE_PATH = Path("./demo-input.txt") if is_debug else Path("./input.txt")


def read_file(file_path: str) -> str:
    with open(file_path, "r") as f:
        return f.read()


def sol_1(content: str) -> int:
    sum: int = 0
    first: int = 0
    last: int = 0

    for line in content.split("\n"):
        if not line:
            continue
        # Look number from the front
        print(line)
        for i in range(0, len(line) - 1):
            if line[i].isdigit():
                first = int(line[i])
                break

        # Look number from the back
        for i in range(len(line) - 1, 0, -1):
            if line[i].isdigit():
                last = int(line[i])
                break

        number = first * 10 + last
        print(f"Found number {number}")
        sum += number

    return sum


def main():
    content = read_file(str(FILE_PATH))
    sum = sol_1(content)
    print(f"Sum: {sum}")


if __name__ == "__main__":
    main()
    pass
