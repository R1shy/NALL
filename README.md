# NALL (Not Another Logging Library)
this is in fact another logging Library


# Usage

```rust
use NALL::log::log;
use NALL::clog::clog;
fn main() {


log(NALL::levels::LogLevel::Info, "Hi I am information".to_string());
log(NALL::levels::LogLevel::Warn, "Hi, I am a warning".to_string());
log(NALL::levels::LogLevel::Err, "Hi, I am an error".to_string());
log(NALL::levels::LogLevel::Fatal, "Goodbye :)".to_string());
clog(NALL::levels::LogLevel::Info, "Hi I am information".to_string(), None);
clog(NALL::levels::LogLevel::Warn, "Hi, I am a warning".to_string(), None);
clog(NALL::levels::LogLevel::Err, "Hi, I am an error".to_string(), None);
clog(NALL::levels::LogLevel::Fatal, "Goodbye :)".to_string(), None);

}
```
for clog check [this ansi color cli](https://github.com/fidian/ansi) for possible color values

# Installation

```bash

# existing:
    cargo add NALL

# new project:
    cargo new my-project
    cd my-project 
    cargo add NALL
```
