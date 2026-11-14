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
    - [An underscore makes all the difference](#-an-underscore-makes-all-the-difference)
    - [Shadowing is not dropping](#-shadowing-is-not-dropping)
  - [Section: Collections and evaluation](#section-collections-and-evaluation)
    - [One expression, three shared handles](#-one-expression-three-shared-handles)
    - [Zero elements, one side effect](#-zero-elements-one-side-effect)
  - [Section: Hidden details](#section-hidden-details)
    - [The constant that forgets its changes](#-the-constant-that-forgets-its-changes)
    - [The dead branch that won't compile](#-the-dead-branch-that-wont-compile)
  - [Section: Methods and types](#section-methods-and-types)
    - [The method that wasn't called](#-the-method-that-wasnt-called)
    - [Two identical-looking, incompatible types](#-two-identical-looking-incompatible-types)
  - [Section: Closures and captures](#section-closures-and-captures)
    - [A closure that moves only half](#-a-closure-that-moves-only-half)
    - [Moving once, calling twice](#-moving-once-calling-twice)
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

### ▶ The dead branch that won't compile

```rust
fn main() {
    if cfg!(any()) {
        nonexistent_function();
    }

    println!("Hello");
}
```

**Compiler result:**

```text
error[E0425]
```

The condition `cfg!(any())` is always `false`, so the branch can never execute. Nevertheless, the program fails to compile.

By contrast, this program compiles successfully:

```rust
#[cfg(any())]
fn unused() {
    nonexistent_function();
}

fn main() {
    println!("Hello");
}
```

**Output:**

```text
Hello
```

#### 💡 Explanation:

Rust provides two similar-looking conditional compilation mechanisms with fundamentally different behavior:

- **`cfg!(...)` is a macro** that evaluates a configuration predicate and produces a boolean. It does not remove code.
- **`#[cfg(...)]` is an attribute** that conditionally includes or excludes code during compilation.

Although `cfg!(any())` always evaluates to `false`, the compiler still resolves names and type-checks the contents of the unreachable branch.

Conversely, `#[cfg(any())]` removes the annotated function before type checking, so the reference to `nonexistent_function()` causes no error.

**Sources:**
- [`examples/compile_fail/cfg_does_not_remove_code.rs`](examples/compile_fail/cfg_does_not_remove_code.rs)

**References:**
- [`cfg!` macro](https://doc.rust-lang.org/std/macro.cfg.html)
- [`cfg` attribute](https://doc.rust-lang.org/reference/conditional-compilation.html)

---

## Section: Methods and types

### ▶ The method that wasn't called

```rust
struct Device;

trait Run {
    fn run(&self);
}

impl Device {
    fn run(&mut self) {
        println!("inherent");
    }
}

impl Run for Device {
    fn run(&self) {
        println!("trait");
    }
}

fn main() {
    let mut device = Device;
    device.run();
    Device::run(&mut device);
}
```

**Output:**

```text
trait
inherent
```

#### 💡 Explanation:

Rust searches receiver types in order, considering `Device`, then `&Device`, then `&mut Device`.

The trait method accepts `&self`, so it is found before the inherent method requiring `&mut self`. Consequently, `device.run()` calls the trait implementation, even though `Device` has an inherent method with the same name.

The explicit call `Device::run(&mut device)` selects the inherent method.

**Sources:**
- [`examples/method_resolution_surprise.rs`](examples/method_resolution_surprise.rs)

**References:**
- [Rust Reference — Method-call expressions](https://doc.rust-lang.org/reference/expressions/method-call-expr.html)

### ▶ Two identical-looking, incompatible types

```rust
fn first() -> impl Iterator<Item = u8> {
    0..3
}

fn second() -> impl Iterator<Item = u8> {
    0..3
}

fn main() {
    let _iterators = vec![first(), second()];
}
```

**Compiler result:**

```text
error[E0308]
```

Both functions return the same concrete type, `Range<u8>`, and expose the same return signature. Yet their results cannot be stored in the same vector.

#### 💡 Explanation:

Return-position `impl Trait` introduces an opaque type whose concrete implementation is hidden from callers.

Each function declares its own distinct opaque return type, even if both happen to use the same underlying concrete type.

Therefore, although both `first()` and `second()` return `Range<u8>` internally, their opaque return types are distinct from the caller's perspective.

The `vec!` macro requires elements to have a common concrete type, resulting in a type mismatch.

If the functions instead explicitly returned `std::ops::Range<u8>`, the vector would compile:

```rust
fn first() -> std::ops::Range<u8> {
    0..3
}

fn second() -> std::ops::Range<u8> {
    0..3
}

fn main() {
    let _iterators = vec![first(), second()];
}
```

The important distinction is that `impl Trait` does not mean "any type implementing this trait." It represents a specific opaque type chosen by the function defining it.

**Sources:**
- [`examples/compile_fail/distinct_opaque_return_types.rs`](examples/compile_fail/distinct_opaque_return_types.rs)

**References:**
- [Rust Reference — impl Trait](https://doc.rust-lang.org/reference/types/impl-trait.html)

## Section: Closures and captures

### ▶ A closure that moves only half

```rust
struct Message {
    subject: String,
    body: String,
}

fn main() {
    let message = Message {
        subject: String::from("Hello"),
        body: String::from("World"),
    };

    let print_subject = move || {
        println!("{}", message.subject);
    };

    println!("{}", message.body);
    print_subject();
}
```

**Output:**

```text
World
Hello
```

#### 💡 Explanation:

In Rust 2021 and later, closures can capture individual struct fields rather than entire variables.

Although the closure uses `move`, it only captures `message.subject`, not the entire `message` struct.

Both fields contain `String` values, which do not implement `Copy`. Nevertheless, `message.body` remains accessible after the closure is created.

This behavior is known as *disjoint field capture* or *precise closure capture*.

There are exceptions to this capture precision, including certain types implementing `Drop`.

**Sources:**
- [`examples/partial_closure_capture.rs`](examples/partial_closure_capture.rs)

**References:**
- [Rust Edition Guide — Disjoint capture in closures](https://doc.rust-lang.org/edition-guide/rust-2021/disjoint-capture-in-closures.html)

### ▶ Moving once, calling twice

```rust
fn call_twice(greet: impl Fn(&str)) {
    greet("Alice");
    greet("Bob");
}

fn main() {
    let prefix = String::from("Hello");
    let greet = move |name: &str| {
        println!("{prefix}, {name}");
    };

    call_twice(greet);
}
```

**Output:**

```text
Hello, Alice
Hello, Bob
```

Despite explicitly capturing ownership of `prefix` with `move`, the closure satisfies the `Fn` trait and can be called multiple times.

#### 💡 Explanation:

The `move` keyword determines how a closure captures variables from its environment. It does not determine how many times the closure can be called.

In this example:

- The closure takes ownership of `prefix`, since it is a `String` and does not implement `Copy`.
- Each invocation merely borrows `prefix` to format the output.
- The closure therefore implements `Fn`, allowing it to be called repeatedly.

The closure's call traits depend on what its body does with captured values:

- `Fn`: Does not mutate or move out of captured values.
- `FnMut`: May mutate captured values, but does not move them out.
- `FnOnce`: May move captured values out, consuming them.

If the closure body were changed to call `drop(prefix)`, it would move the captured `String` out of the closure when invoked.

Such a closure would implement only `FnOnce` and would no longer satisfy the `Fn` requirement of `call_twice`.

**Sources:**
- [`examples/move_closure_called_twice.rs`](examples/move_closure_called_twice.rs)

**References:**
- [Rust Reference — Closure call traits](https://doc.rust-lang.org/reference/types/closure.html#call-traits-and-coercions)

## Run the examples

Requires a stable Rust compiler with edition 2024 support (Rust 1.85+) and Bash.

```bash
# Run a single snippet
rustc --edition=2024 examples/wildcard_preserves_ownership.rs -o /tmp/wtf-rust-example
/tmp/wtf-rust-example

# Check all outputs and the intentional compile-fail examples
bash scripts/verify.sh
```

## Contributing

Submissions should provide genuinely surprising **Rust-specific** behavior with a minimal snippet, its observed/guaranteed result, a precise explanation, and a language or standard-library reference. See [CONTRIBUTING.md](CONTRIBUTING.md).

Avoid undefined behavior, compiler-specific accidents presented as language guarantees, and general programming surprises unrelated to Rust.

## 🎓 License

MIT. See [LICENSE](LICENSE).
