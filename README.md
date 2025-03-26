# NALL (Not Another Logging Library)

this is in fact another logging Library


# Usage

```rust
use NALL::plog::log;

fn main() {


log(NALL::levels::LogLevel::Info, "Hi I am an info".to_string())
// there are also Warn, Err, Fatal
// NOTE: using Fatal will kill your program with exit code 1!
}
```

# Installation

```bash

# existing:
    cargo add NALL

# new project:
    cargo new my-project
    cd my-project 
    cargo add NALL
```
