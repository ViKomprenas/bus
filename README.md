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
