# Async Code in Rust 🦀

- Async code can wait on a large amount of futures at the same time, without blocking the thread.
- Async code only ever executes serially, on a single thread.
- Async waits on a `Future` to complete, and then continue executing.
- Async code is non-blocking, meaning that it doesn't block the thread while waiting for a `Future` to complete.

## Futures

- Futures are a way to represent code that will be executed at a future time.
- Futures are lazily evaluated, they won't be executed until they're awaited.
- Futures are ran on an `Executor`, which is a type that runs futures.
  - Execution of a `Future` can be paused by using `.await`, the `Executor` will then run other `Futures` until they complete or also `.await`.
