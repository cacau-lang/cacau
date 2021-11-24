# Reads stuff and turns them into ASTs

So, this is a WIP. Works only with the following rules: num, integer, float, identifier

To run it, it's like,

```console
echo 1.55 | cargo run num
echo 1.55 | cargo run float

echo 1 | cargo run num
echo 1 | cargo run integer

echo x | cargo run identifier
```
