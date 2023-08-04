# Basic example

This example shows how to make a static website generator that process html body files like:
```html
Hello
```
and wraps them in a template like:
```html
<html>
<head>
    ...
    <title>My Blog</title>
</head>
<body>
    Hello
</body>
</html>
```

## Usage

```sh
cargo run
```
or, using `cargo-watch`:
```sh
cargo watch -x run
```