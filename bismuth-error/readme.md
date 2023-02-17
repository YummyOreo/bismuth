# This need to be rewritten, here is my idea:
The error system will revolve around a single enum:
```rust
pub enum Error<'a>  {
    Recoverable(&'a dyn Recover),
    Unrecoverable(anyhow::Error),
}
```

### Recoverable:
The `Recoverable` trait will have 1 functions:
```rust
// will return the good value "t" if it sucseeds, or a error to panic w/
// E will be anything with anyhow::Error
fn recover(&self) -> Result<T, anyhow::Error>;
```
### Unrecoverable:
This will just be a error to be paniced
### `Recover` trait:
The recover trait will be implemented on both the `Error` enum and the `Result` enum:
#### `Error`:
This will be called in Result::try_recover is called.
```rust
// this is not tested or good
impl<T> Recover<T> for Error<T> {
    fn try_recover(self) -> T {
        match self {
            Self::Recoverable(e) => match e.recover() {
                Ok(t) => t,
                Err(e) => panic!("{e}"),
            },
            Self::Unrecoverable(e) => panic!("{e}"),
        }
    }
}
```
#### `Result`:
```rust
impl<T> Recover<T> for Result<T, Error<T>> {
    fn try_recover(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => e.try_recover(),
        }
    }
}
```

### Using this type:
Here is a example on how to use this type:
> It is not perfect because there is some code duplication w/ the "good" type:
```rust
pub fn test_2() {
    let t = test().unwrap_err();
    let t = match t {
        Error::Recoverable(e) => e.get_recoverd(),
        Error::Unrecoverable(e) => panic!("{e}"),
    };
}

pub fn test() -> Result<bool, TestError<bool>> {
    Err(Error::Recoverable(Box::new(TestError {})))
}
```
