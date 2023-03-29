# Derive Getters

```rust
use derive_get::Getters;

#[derive(Getters)]
struct Example {
  field_a: String,
  field_b: u8,
  #[copy]
  field_c: u8,
  #[skip]
  field_d: String,
}
```

Expands to:

```rust
impl Example {
  pub fn field_a(&self) -> &String {
    &self.field_a
  }
  pub fn field_b(&self) -> &u8 {
    &self.field_b
  }
  pub fn field_c(&self) -> u8 {
    self.field_c
  }
}
```
