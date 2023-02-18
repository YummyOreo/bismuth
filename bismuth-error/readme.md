# This need to be rewritten, here is my idea:
The error system will revolve around a single enum:
```rust
pub enum Error<'a>  {
    Recoverable(Box<dyn Recover>),
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
First, there will be a type called `Result` that you can import:
```rust
type Result<T, E = Test<T>> = core::result::Result<T, E>;
```
There is also a helper function `recover`. This will box your error for you:
```rust
impl<T> Error<T> {
    pub fn recover<E: Recoverable<T> + 'static>(error: E) -> Self {
        Self::Recoverable(Box::new(error))
    }
}
```
This allows you to just use it like this:
```rust
pub fn test_2() {
    let t = test().try_recover();
}

pub fn test() -> Result<bool> {
    Err(Error::recover(TestError {}))
}
```
