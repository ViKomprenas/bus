# `bus`

`bus` is a lightweight tool used to _bus_ the output of a program to a temporary file.
It's mainly useful when you want to pipe something into a program that expects a file argument,
and won't accept reading from stdin.
For example, your pager, which of course needs to read commands from stdin.

`bus` creates a temporary file, funnels its stdin into it, and then launches the program supplied as its arguments.
For example, if you invoke `bus less -F`,
the command that would be executed would resemble `less -F /tmp/somefile.txt`.

`bus`'s only dependency is `mktemp`.
It uses that to generate the temporary file name.

# Usage

```
bus [command] [args...]
```

That's all. If you supply a command, `bus` will invoke it;
otherwise, `bus` will invoke `$PAGER` by default.
If one of the command's arguments contains `{}`,
`bus` will replace this with the file name;
otherwise, the file name is the last argument.
You can also use `{}` to force the file name to appear multiple times,
if you like.

# Examples

```
# Use a command to generate markdown, then run it through pandoc to generate a pdf
thingy --markdown | bus pandoc '{}' -sf markdown -t latex -o output.pdf

# Search for a string in a file, emphasize it, and open the result in your pager
grep string file | sed 's/string/STRING/g' | bus

# Load the output of a command into vim for manual editing and saving
thingy --generic-example | bus vim
```
