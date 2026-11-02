<h1 align="center">What the f*ck Rust! 😱</h1>
<p align="center">Exploring and understanding Rust through surprising snippets.</p>

Inspired by [WTF Python](https://github.com/satwikkansal/wtfpython). Rust is designed to make many classes of errors impossible, but it still has language rules that can produce unexpected results, especially if you're coming from another language.

This collection explains those results with small, reproducible examples. It is **not a quiz, a course, or a list of compiler errors**. Each example contains the code, its output or compilation result, and an explanation of the rule behind it.

# Table of Contents

<!-- toc -->

- [Structure of the Examples](#structure-of-the-examples)
  - [▶ Some fancy Title](#-some-fancy-title)
- [Usage](#usage)
- [👀 Examples](#-examples)
  - [Section: Ownership and bindings](#section-ownership-and-bindings)
    - [An underscore makes all the difference](#an-underscore-makes-all-the-difference)
    - [Shadowing is not dropping](#shadowing-is-not-dropping)
  - [Section: Collections and evaluation](#section-collections-and-evaluation)
    - [One expression, three shared handles](#one-expression-three-shared-handles)
    - [Zero elements, one side effect](#zero-elements-one-side-effect)
  - [Section: Hidden details](#section-hidden-details)
    - [The constant that forgets its changes](#the-constant-that-forgets-its-changes)
- [Run the examples](#run-the-examples)
- [Contributing](#contributing)
- [🎓 License](#-license)

<!-- tocstop -->

# Structure of the Examples

> ## Section: (if necessary)
>
> ### ▶ Some fancy Title
>
> ```rust
> // A short Rust snippet showing an initially surprising behavior.
> ```
>
> **Output and/or Compiler result:**
>
> ```text
> Some unexpected output
> `error[E0382]: borrow of moved value: message`
> ```
>
> (Optional): Description of the unexpected output.
>
> #### 💡 Explanation:
>
> - Brief explanation of what's happening and why it is happening.
>
> **Sources:**
> - [`examples/wildcard_preserves_ownership.rs`](examples/wildcard_preserves_ownership.rs)
>
> **References:**
> - [Rust Reference — Wildcard pattern](https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern)
>

## Usage

Read the examples in any order. The snippets are standalone Rust programs and use only the standard library. The accompanying files in [`examples/`](examples/) can be run locally; [`expected/`](expected/) records their standard output.

The examples target **Rust edition 2024** and the stable Rust toolchain (Rust 1.85 or later). Output is specified where Rust guarantees the relevant behavior; compiler warnings and exact diagnostic wording may vary.

---

# 👀 Examples

## Section: Ownership and bindings

### ▶ An underscore makes all the difference

Consider the following Rust program:

```rust
fn main() {
    let message = String::from("hello");
    let _ = message;
    println!("{message}");
}
```

**Output:**

```text
hello
```

Nothing particularly surprising so far. But what happens if we replace `_` with `_ignored`?

```diff
fn main() {
    let message = String::from("hello");
-   let _ = message;
+   let _ignored = message;
    println!("{message}");
}
```

**Compiler result:**

`error[E0382]: borrow of moved value: message`

The first program compiles successfully, while the second does not.

#### 💡 Explanation:

In Rust, `_` and identifiers beginning with `_` have different semantics:

- **`_` is a wildcard pattern.** It does not create a binding and does not move or copy the matched value. Consequently, `message` remains available.
- **`_ignored` is a variable binding.** Its leading underscore suppresses unused-variable warnings, but otherwise it behaves like an ordinary variable.
- Since `String` does not implement `Copy`, assigning `message` to `_ignored` transfers ownership. The original variable can no longer be used.

The distinction becomes important when deliberately ignoring values. Using `_` discards the binding without taking ownership, whereas `_name` can take ownership of a value and cause it to be dropped when the binding's scope ends.

**Sources:**
- [`examples/wildcard_preserves_ownership.rs`](examples/wildcard_preserves_ownership.rs)
- [`examples/compile_fail/underscore_binding_moves_ownership.rs`](examples/compile_fail/underscore_binding_moves_ownership.rs)

**References:**
- [Rust Reference — Wildcard pattern](https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern)
- [The Rust Programming Language — Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

### ▶ Shadowing is not dropping

```rust
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let _guard = Guard("first");
    let _guard = Guard("second");
    println!("end");
}
```

**Output:**

```text
end
drop second
drop first
```

#### 💡 Explanation:

- Both `let` statements create separate bindings, even though they use the same name.
- Shadowing does not drop the old value immediately. Both guards are dropped at the end of the scope, in reverse declaration order.
- For a resource that must be released early, use an inner scope or an explicit `drop(guard)` rather than relying on shadowing.

**Sources:**
- [`examples/shadowing_is_not_dropping.rs`](examples/shadowing_is_not_dropping.rs)

**References:**
- [Rust Reference — destructors and scopes](https://doc.rust-lang.org/reference/destructors.html)

---

## Section: Collections and evaluation

### ▶ One expression, three shared handles

```rust
use std::{cell::Cell, rc::Rc};

fn main() {
    let cells = vec![Rc::new(Cell::new(0)); 3];
    cells[0].set(7);
    println!("{}", cells[2].get());
}
```

**Output:**

```text
7
```

#### 💡 Explanation:

- `vec![expression; count]` evaluates `expression` once and clones that value to fill the vector.
- Cloning `Rc<T>` clones the **reference-counted handle**, not the underlying allocation. Consequently, all three elements refer to the same `Cell`.
- To create three independent allocations, use:
  ```rust
  let cells: Vec<_> = (0..3).map(|_| Rc::new(Cell::new(0))).collect();
  ```

**Sources:**
- [`examples/clone_the_handle.rs`](examples/clone_the_handle.rs)

**References:**
- [`vec!` macro](https://doc.rust-lang.org/std/macro.vec.html)
- [`Rc` cloning](https://doc.rust-lang.org/std/rc/struct.Rc.html)

---

### ▶ Zero elements, one side effect

```rust
fn main() {
    let values = vec![println!("surprise!"); 0];
    println!("length: {}", values.len());
}
```

**Output:**

```text
surprise!
length: 0
```

#### 💡 Explanation:

- The repetition form `vec![expression; count]` evaluates the expression **even when the count is zero**.
- The produced value is not placed in the vector and is dropped. The vector itself remains empty.
- Be aware of side effects or resource acquisition in repeated expressions.

**Sources:**
- [`examples/zero_length_side_effect.rs`](examples/zero_length_side_effect.rs)

**References:**
- [`vec!` macro](https://doc.rust-lang.org/std/macro.vec.html)

---

## Section: Hidden details

### ▶ The constant that forgets its changes

```rust
use std::sync::atomic::{AtomicU8, Ordering};

const COUNTER: AtomicU8 = AtomicU8::new(1);

fn main() {
    COUNTER.store(2, Ordering::Relaxed);
    println!("{}", COUNTER.load(Ordering::Relaxed));
}
```

**Output:**

```text
1
```

#### 💡 Explanation:

- A `const` is a named value, not a single shared storage location. Each use denotes that value rather than accessing one persistent atomic object.
- Mutating one instance therefore has no effect on the next use. Rust may warn about interior mutation of constants.
- If the goal is shared atomic state, use `static COUNTER: AtomicU8 = AtomicU8::new(1);` instead.
- Modern Rust also has a compiler lint, const_item_interior_mutations, for this exact problem.

**Sources:**
- [`examples/const_is_not_shared.rs`](examples/const_is_not_shared.rs)

**References:**
- [Rust Reference — constant items](https://doc.rust-lang.org/reference/items/constant-items.html)
- [The rustc book — CONST_ITEM_INTERIOR_MUTATIONS](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#const-item-interior-mutations)

---

## Run the examples

Requires a stable Rust compiler with edition 2024 support (Rust 1.85+) and Bash.

```bash
# Run a single snippet
rustc --edition=2024 examples/wildcard_preserves_ownership.rs -o /tmp/wtf-rust-example
/tmp/wtf-rust-example

# Check all outputs and the intentional compile-fail example
bash scripts/verify.sh
```

## Contributing

Submissions should provide genuinely surprising **Rust-specific** behavior with a minimal snippet, its observed/guaranteed result, a precise explanation, and a language or standard-library reference. See [CONTRIBUTING.md](CONTRIBUTING.md).

Avoid undefined behavior, compiler-specific accidents presented as language guarantees, and general programming surprises unrelated to Rust.

## License

MIT. See [LICENSE](LICENSE).
