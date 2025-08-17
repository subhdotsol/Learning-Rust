
````md
# 🧱 Rust Module System & `use` Keyword Explained

This section summarizes how Rust handles modules, visibility, the `use` keyword, and naming conflicts — as part of my Rust learning journey.

---

## 📦 Module Structure in Rust

Rust organizes code using modules, which help group related functionality together.

You can define modules inline like this:

```rust
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    pub mod serving {
        pub fn take_order() {}
    }
}
````

### ✅ Key Notes:

* Modules (`mod`) and their contents (`fn`, `struct`, etc.) are **private by default**.
* You must use `pub` to expose them outside of their immediate scope.

---

## 📌 Paths in Rust

Rust has two types of paths:

* **Absolute path**: Starts from the crate root using `crate::`
* **Relative path**: Starts from the current module or scope

### Example:

```rust
crate::front_of_house::hosting::add_to_waitlist(); // Absolute path
front_of_house::hosting::add_to_waitlist();        // Relative path
```

---

## 🧠 The `use` Keyword

You can bring functions, structs, or modules into scope using `use`.
This simplifies long paths and makes your code cleaner.

### Example:

```rust
use crate::front_of_house::hosting::add_to_waitlist;

fn eat_at_restaurant() {
    add_to_waitlist(); // Now directly accessible
}
```

### ✅ Tip:

* The `use` keyword brings **items into local scope**, not into the global one.
* This improves readability, especially with deeply nested modules.

---

## 🌳 Nested Paths

You can simplify multiple imports from the same module using **nested paths**:

```rust
use rand::{Rng, CryptoRng, ErrorKind}; // All from rand
```

---

## 🚫 Name Conflict Resolution

Sometimes different modules expose items with the same name. Rust allows you to resolve these using **aliasing** with `as`.

### Without aliasing (conflict potential):

```rust
use std::fmt::Result;
use std::io::Result; // ❌ Conflicts with fmt::Result
```

### With aliasing:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

Now you can use them separately:

```rust
fn format_fn() -> Result { /* ... */ }

fn io_fn() -> IoResult { /* ... */ }
```

---

## 🔒 Privacy Rules

Rust enforces strong encapsulation:

* Everything is **private by default**
* You must use `pub` to expose functions, structs, or modules
* `use` can only access **public items**

Trying to access a non-public item like this will result in a **compiler error**:

```rust
crate::front_of_house::hosting::add_to_waitlist(); // ❌ If not public
```

Make sure to mark both the module and the function as `pub`:

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

---

## ✅ Summary

| Concept         | Description                                        |
| --------------- | -------------------------------------------------- |
| `mod`           | Defines a module                                   |
| `pub`           | Makes functions or modules public                  |
| `use`           | Brings items into scope                            |
| Absolute path   | Starts from `crate::`                              |
| Relative path   | Starts from current scope/module                   |
| Nested paths    | Cleanly import multiple items from the same module |
| Aliasing (`as`) | Resolve naming conflicts between modules           |

---

📚 Rust’s module system can seem strict at first, but it gives you **fine-grained control** over visibility and access — helping you write clear and safe code.

---

> "Encapsulation is not just about hiding data — it's about exposing only what matters."
> — *Rustacean Wisdom*

🦀 **Happy learning Rust!**
