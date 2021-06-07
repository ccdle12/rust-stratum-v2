# Fuzzing


## Install

- Install dependencies on Debian based systems:

```
sudo apt install build-essential binutils-dev libunwind-dev
```

- Install honggfuzz

```
cargo install --force honggfuzz
```

## Run

```
export CPU_COUNT=1 # replace as needed
export HFUZZ_BUILD_ARGS="--features honggfuzz_fuzz"
export HFUZZ_RUN_ARGS="-n $CPU_COUNT -N 100000 --exit_upon_crash"
export HFUZZ_DEBUGGER=rust-gdb
```

View all the fuzzing targets:

```
ls ./src/bin
```

Run a fuzzing target:

```
cargo hfuzz run <target-name>
```

Run in debug environment to view the crash:

```
cargo hfuzz run-debug <target-name> hfuzz_workspace/*/*.fuzz
```
