# slang-fuzz

This is a fuzzer for the [slang](https://github.com/NomicFoundation/slang) parser. It currently only tests that the parsed syntax tree is equivalent to its input when "unparsed" to a string aka [roundtrip property](https://blog.ssanj.net/posts/2016-06-26-property-based-testing-patterns.html), but it also finds inputs that cause the parser to panic.

## Usage

```
./init_corpus.sh # download Solidity files to seed the corpus
cargo fuzz run roundtrip_parse
```

To get started with cargo fuzz, see [cargo-fuzz](https://rust-fuzz.github.io/book/introduction.html).