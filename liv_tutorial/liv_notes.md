def main():
    pass

^ Defines a function called main - usually the entry point of the script (where main functions sit)

if __name__ == "__main__":
    pass

^ __name__ is a special variable which is dynamically defined by the Python interpreter. The variable is created automatically by the Python interpreter; if the script is being run directly in the terminal by Python and __name__ isn't otherwise defined, it will automatically evaluate to __main__. We check whether __name__ is equal to "__main__" to make sure that if this script is imported into another script, that __name__ in the imported script will not be equal to __main__ (as the imported script is not being run directly in the terminal so will evaluate to the name of the script being imported (e.g. bob). This means the functions from the imported script will be usuable in the script being run, without the bulk of the imported script being run as well.

If __name__ is equal to __main__, that means the file __name__ is in is the main file being run by the Python interpreter.

(If getting confused by the above, go and look at Bob and Alice!)

```python
def read_file(file_path: str) -> List[str]:
    with open(file_path) as fp:
        pass
```

instead of:

```python
 def read_file(file_path):
    fp = open(file_path)
    # [some code here]
    fp.close
```

  Python context managing instead of regular code: First example - With the value the funtction open returns, which is stored inside the variable fp, run the code below (indented code which belongs to the line containing the open function - the context). Once that code finishes, automatically close fp (the file at the end of the address contained in fp which points to a bit of the hard drive where the data is contained). Compare with second example - means we don't have to remember to close the file at the end!

  Type hints: In example one, file_path is suggested as being of type str - the compiler totally ignores this (lol) - it's purely there for human readers' info and linting tools. The return type is specified as well: -> List[str] says that the output of the function should be a list of strings.

```python
    lines = []
    with open(file_path) as fp:
        for line in fp:
            lines.append(line)
    return lines
```

[] is an empty list
Because the variable lines is an empty list, we need to add the line values from the for loop (which is iterating over each line of the input data) into the lines list. The append function adds the contents of each line of the input data into the list inside the lines variable. We return the lines variable outside the for loop so the contents of lines becomes available to use in other functions.

The Lines variable is defined in a higher block so it is both visible to the code running in the sub-block (containing with open etc) but also visible to the return statement on the same level, so the variable can be returned and used outside of the function.

File paths: The code only cares about where it is run from - not where it actually is. (Ofc exceptions exist... thanks Jamie...) But e.g. if we are sat in the terminal at the file AOC_22, we would need to specify all folders down to the file onwards from that point in the hierarchy.

    Using global variable for the file path: The path to the input data is a constant and not something we would want to change. Better to define this as a global variable at the top of the code:

    ```python
    FILE_PATH = "days/day_1/input_data/input.txt"

    def main():
        calorie_counts = read_file(FILE_PATH)
    ```

When you want to run your code:
    Make sure to remove the pass underneath the if __name__ statement and call the main function!! (Otherwise unsurprisingly, nothing will happen...) The if __name__ statement also needs to be at the bottom of the code, otherwise anything below it won't be defined (as all the main functionality is above it).

    ```python
    if __name__ == "__main__":
    main()
    ```
Where files have new line characters, don't forget to trim the whitespace inside the for loop for each string:

```python
for line in fp:
            line = line.rstrip()
            lines.append(line)
```
