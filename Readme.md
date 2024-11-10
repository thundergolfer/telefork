# telefork

This is a fork of the [original project by Tristan Hume](https://github.com/trishume/telefork).
It is focused on modifying the code to provide a CRIU-like interface and attempting the "even crazier
ideas" that Tristan suggested in the [original blogpost](https://thume.ca/2020/04/18/telefork-forking-a-process-onto-a-different-computer/).

# Usage

```
Usage: telefork [OPTIONS] <COMMAND>

Commands:
  dump     Dump a running process to a file for later restoration
  restore  Restore a process from a dumped file
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose <VERBOSE>  Verbosity level (can be specified multiple times) [default: 0]
  -h, --help               Print help
  -V, --version            Print version
```

Basically it's like the `fork()` syscall except it can fork a process onto a
different computer. It does this using a bunch of ptrace magic to serialize
the memory mappings of the process, stream them over a pipe and recreate them
on the other end along with the registers and some other process state.

## Example

```bash
# 1. Start a process to dump.
./examples/dumpme.sh &
# 2. Grab its PID
PID=$!
# 3. Wait for it to progress a bit before dumping
sleep 3
# 4. Dump the process to a file
telefork dump $PID checkpoint.img
# 5. Restore the process from the file and wait for it to finish!
telefork restore checkpoint.img
```

`dumpme.sh` is a simple script that counts slowly to 10 with `echo` and `sleep`.
In the example above you can save the process once it's counted to 3 and then
later restore it and see it finish counting to 10 before exiting successfully!

This example uses all the features originally implemented by Tristan _plus_ basic
file descriptor restoration, needed because a bash process will read from its source
code file before exiting.

# How it works

Read the code in `src/lib.rs!`. Tristan wrote it all in **one file with
tons of comments** in an order meant to read top to bottom. Hopefully it should
be easy enough to understand what it's doing, provided some familiarity with
systems programming concepts.

# Examples

- `basic` and `load`: Save and restore a process state to a file
- `dump` (_new_ ✨): Dump a running process to a file
- `teleserver` and `teleclient`: Fork a process to a remote server
- `yoyo_client` and `yoyo_client_raw`: Execute a closure on a remote server by teleforking there and back
- `smallpt`: Use `yoyo` to run a path tracing render on a remote server from a local executable.
