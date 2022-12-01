def main():
    pass

^ Defines a function called main - usually the entry point of the script (where main functions sit)

if __name__ == "__main__":
    pass

^ __name__ is a special variable which is dynamically defined by the Python interpreter. The variable is created automatically by the Python interpreter; if the script is being run directly in the terminal by Python and __name__ isn't otherwise defined, it will automatically evaluate to __main__. We check whether __name__ is equal to "__main__" to make sure that if this script is imported into another script, that __name__ in the imported script will not be equal to __main__ (as the imported script is not being run directly in the terminal so will evaluate to the name of the script being imported (e.g. bob). This means the functions from the imported script will be usuable in the script being run, without the bulk of the imported script being run as well.

If __name__ is equal to __main__, that means the file __name__ is in is the main file being run by the Python interpreter.

(If getting confused by the above, go and look at Bob and Alice!)
