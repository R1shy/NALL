# NALL (Not Another Logging Library)
this is in fact another logging Library


# Usage

```rust
use NALL::log::log;
use NALL::clog::clog;
fn main() {


log(NALL::levels::LogLevel::Info, "Hi I am an info".to_string())
// there are also Warn, Err, Fatal
// NOTE: using Fatal will kill your program with exit code 1!
clog(NALL::levels::LogLevel::Info, "Hi I am an info".to_string());
// you can also have colored messages with the color feature
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

# add clog:
    cargo add NALL --features color

```
