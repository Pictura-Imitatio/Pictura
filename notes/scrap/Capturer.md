# Struct scrap::x11::[[Capturer]]
---
```rust
pub struct Capturer { /* fields omitted */ }
```
# Methods
---
## impl [[Capturer]]
```rust
pub fn new(display: Display) -> Result<Capturer>
```

```rust
pub fn display(&self) -> &Display
```

```rust
pub fn frame<'b>(&'b mut self) -> &'b [u8]
```
# Trait Implementations
---
## impl [[Drop]] for [[Capturer]]