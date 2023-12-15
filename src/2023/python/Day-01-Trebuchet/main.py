import dotenv
import os
from pathlib import Path

dotenv.load_dotenv()

is_debug = True if os.environ.get("DEMO_APP", "1") else False
FILE_PATH = Path("./demo-input.txt") if is_debug else Path("./input.txt")


def read_file(file_path: str) -> str:
    with open(file_path, "r") as f:
        return f.read()


def main():
    content = read_file(str(FILE_PATH))
    print(content)


if __name__ == "__main__":
    main()
    pass
