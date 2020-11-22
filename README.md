# felt - Defines aliases and executes them.

felt is a simple command runner, written in Rust.

## Install

Install [Rust](https://www.rust-lang.org/learn/get-started) first. Then you can use `cargo` command to install `felt`.

`cargo` command is bundled with Rust.

```
cargo install felt
```

## Supported OS

|Family               |Supported|Note|
|:--------------------|:-------:|:---|
|Windows              | 🚧      |    |
|macOS(Intel)         | ✅      |    |
|macOS(Apple Silicon) | ❓      |    |
|Linux                | ✅      |    |

## Usage

Put a `.feltrc.toml` file in your home directory, which means `~/.feltrc.toml` file.

```toml
[felt]
root = true
node_modules = true

[command]
hello = "echo Hello! I\\'m Felt!"
nid = "npm install --save-dev"
```

You have defined `hello` command above. Let's execute it

```sh
felt hello
```

You will get output `Hello! I'm Felt!`.

You can also execute them with args.

```sh
felt nid esbuild react react-dom
```

## Execute node_modules

`felt` can execute command in `./node_modules/.bin/` directly. If you want this feature, set `node_modules` in `.feltrc.toml` to `true`.

```sh
# You may have some Node.js binaries.
npm install esbuild

# This execute ./node_modules/.bin/esbuild
felt esbuild index.js --minify --oufile build/index.js
```

**Notice:** Currently, `felt` searches `node_modules` in just current execution directory. Even if other related directory has `node_modules`, `felt` cannot find it.

## .feltrc.toml

`.feltrc.toml` is config file for `felt`. It may be put in home directory.

```toml
[felt]
# If root is true, felt doesn't see parent directory's .fetlrc.toml
# If this file is in home, you should set root to true.
# You can omit this line when it is false.
root = true

# If node_modules is true, felt can run binaries in ./node_modules/.bin/
# If false, disable the feature.
# If omit this line, its value is inherit from parent, or false when no parents set any value.
node_modules = true

[command]
# You can define yor command aliases!

# felt redis-up
redis-up = "docker run -it --rm -p 6379:6379 redis"

# felt countfiles
countfiles = "ls -l | wc -l"
```

## Local .feltrc.toml

`felt` uses `.feltrc.toml` in below order.

1. current directory
1. parent directory
1. parent of parent...(recursively)

If current directory is out of home(e.g. `/tmp/`), append `~/.feltrc.toml` to last of above list.

If `felt` found `root = true` file. `felt` stops traverse there.

For example, home `.feltrc.toml` is:

```toml
[felt]
root = true
node_modules = false

[command]
hello = "echo Hello"
hello2 = "echo Hi"
```

And `~/myproject/.feltrc.toml` is:

```toml
[felt]
node_modules = true

[command]
hello = "Helloooooooooooooooo"
```

You can:

```sh
cd ~/myproject
felt hello
felt hello2
```