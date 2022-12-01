print("ALICE - START")
print(f"__name__ --> {__name__}")

import bob  # noqa: E402


def whisper(subject):
    print("***" + subject + "***")


if __name__ == "__main__":
    whisper("hello world")
    bob.exclaim("hello again")
