# whoisgod

Tells you who the God is.

## Usage

```bash
whoisgod --religion islam
# or
whoisgod -r christianity
```

You can also use `RELIGION` environment variable to set your religion flag.  

```bash
export RELIGION=islam
whoisgod # Allah
```

Right now we only have Islam and Christianity as valid religions.

## Installation

Repo has a nix flake so you can use it via *Nix3*.

```bash
nix run github:rootrim/whoisgod
```

If you don't use *Nix3*, you can install it by source code. We use *mold* as linker
so install that before installation.

## Contribution

If you want to add religions or fix any misunderstandings about religions in  
the code, feel free to contribute. It should be easy as pie.
