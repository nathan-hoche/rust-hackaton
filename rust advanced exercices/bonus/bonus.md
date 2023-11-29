# Mini-project: Command-line Dictionary

- The program should first load a dictionary file into a HashMap.
- The file format could be a simple text file with lines in the format "word:definition".
- Error handling should be used to handle potential I/O errors and parsing errors.
- The user should then be presented with a command-line interface where they can enter commands to look up words, add new words, remove words, or save the dictionary.
- Looking up a word should display the word's definition or an error message if the word isn't in the dictionary.
- Adding a word should prompt the user to enter the word and its definition, then add them to the dictionary.
- Removing a word should remove the word from the dictionary.
- Saving the dictionary should write the current state of the dictionary back to the file.
- The program should loop until the user chooses to exit.
- Use enums and match statements to handle the different commands.
- Structs can be used to encapsulate the dictionary data and associated methods.
