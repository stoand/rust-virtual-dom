# Rust Virtual Dom

Asm.js / Webassembly frontend view library based on macros.

## Example 

Source:

```rust
let view1 = |inner_text: String| template!(.view1>a[href="https://google.com"]{inner_text});
```

Resulting HTML:

```html
<div class="view1">
<a href="https://google.com">{inner_text}</a>
</div>
```