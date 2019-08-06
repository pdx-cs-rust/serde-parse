# Parse
Bart Massey

This demo shows how to use some of the fancy features of the
`deriving` mode of `serde` to parse JSON. It is modeled on a
sample JSON response from the API of the videogame Destiny™
provided to me by students.

The demo shows how to write data structures compatible with
the given JSON. The `Cargo.toml` enables the `derive`
feature of `serde`. The source uses the renaming features of
`serde` to get Rustic names.

Run with `cargo run <result.json`. Nothing will happen: the
response is parsed and the resulting sructure is discarded.
