# Cheat


This utility displays text-based cheatsheets (that you wrote yourself) in your
terminal.


## Configuration

If you set your `CHEATSHEET_DIR` environment variable, that will be used to
find your cheatsheets. Otherwise `cheat` will look in your home directory.


## Creating cheatsheets

Let's create a cheatsheet for `cheat` as an example.

Create the file `cheat.txt` in your home directory or the directory your
`CHEATSHEET_DIR` environment variable points to (if you set it) and put the
following contents inside:

    USAGE: cheat <subject>

Now run the command `cheat cheat` where the first 'cheat' is the application,
and the second 'cheat' is the filename **without** the `.txt` extension you
want to display.

This should print the contents of the file you just created.


## Running it

    cheat <subject>
