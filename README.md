# `fork(2)`-based continuations for Rust

Just for fun! This is unlikely to be useful or reasonable.

```rust
use forkallcc::call_cc;

fn main() {
    assert_eq!(1, call_cc(|k| {
        k.invoke(1);
        0
    }));
}
```

Based on [cccallcc][] for C++.  See the [original writeup][] for more about
this approach, including its numerous limitations.

[cccallcc]: https://github.com/kmcallister/cccallcc
[original writeup]: http://mainisusuallyafunction.blogspot.com/2012/02/continuations-in-c-with-fork.html

```
$ cargo test

$ ./target/examples/simple
early_return: closure called
early_return: closure returns 1

double_return: closure called
double_return: closure returns 0
double_return: closure returns 1
double_return: no saved continuation

$ ./target/examples/backtrack; sleep 2
17 * 23 = 391

XX------------+
|XXXXXXX|     |
|--+  |X|   | |
|  |  |X| --+ |
|     |XXXXX| |
|-+---+--+-X| |
| |XXX   | XXX|
| |X|X---+-+-X|
|XXX|XXXXXX|XX|
|X+-+-+--|XXX |
|X|   |  |--- |
|XXXX |       |
|---X-+-------|
|   XXXXXXXXXXX
+-------------X
```
