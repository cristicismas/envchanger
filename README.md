# Env Changer | envch

This utility program helps you change between multiple .env's for the same repo.

## Commands:

- #### `envch help` - Displays usage info and available commands.

- #### `envch folder [folder_name]` - Sets the source folder of all the .env files.

- #### `envch list` - Lists all available .env files.

- #### `envch [env_name]` - The name of the file in the selected folder. To view the selected folder type `envch list`.

## Usage:

Firstly, you must have a folder that contains all of your .env files. Then, use `envch folder [folder_name]` to tell Env Changer where that folder is located. After doing this, you can see all of your available .env files by using `envch list`, and you can switch between them using `envch [env_name]`.
