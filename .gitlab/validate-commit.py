# SPDX-FileCopyrightText: (C) 2024 Max Rodriguez
# SPDX-License-Identifier: GPL-3.0-or-later

import re
import sys

"""
Validates the commit string passed, returns
a tuple with the exit code and exit message.
"""
def validate_commit_title(title: str) -> (int, str):
    print(f"Validating commit title:\n\n  {title}\n")

    # Regex patterns should follow the following specification:
    # https://www.conventionalcommits.org/en/v1.0.0/#specification
    split_pattern: re.Pattern = re.compile(r'^(.*): (.*)$')
    category_pattern: re.Pattern = re.compile(r'^(?:[a-z0-9]{2,}[_\-|/]?)+(?:\((?:[a-z0-9]{2,}[_\-|/]?)+\))?!?$')
    summary_pattern: re.Pattern = re.compile(r'^[A-Za-z0-9]\S*(?:\s\S*)+[^.!?,\s]$')

    # Apply split regex
    match = split_pattern.match(title)
    if not match:
        return (1, "Commit title has invalid format. It should be \'<type>[optional scope]: <description>\'")

    category, summary = match.groups()

    # Validate category and summary
    if not category_pattern.match(category):
        return (1, "Invalid commit category tag.\nIt should be completely lowercase " +
                "letters or numbers, at least 2 characters long, other allowed characters are: '|', '-', '_', and '/'." +
                "\nRefer to the specification: https://www.conventionalcommits.org/en/v1.0.0/#specification")

    if not summary_pattern.match(summary):
        return (1, "Invalid commit summary. It should start with a letter or number, " +
                "should be not be too short (less than 2 chars) and should not end with punctuation." +
                 "\nRefer to the specification: https://www.conventionalcommits.org/en/v1.0.0/#specification")

    return (0, "Commit naming convention validation successful. ✔")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python validate_commit_title.py '<commit_title>'")
        sys.exit(1)

    commit_title = sys.argv[1]
    res: (int, str) = validate_commit_title(commit_title);

    print(res[1])
    sys.exit(res[0])

