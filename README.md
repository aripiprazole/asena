## Asena

Incremental/single-pass based compiler, the API can be either used for Single-Pass Compiling and for building LSP, or
things that would need incremental pipelines. It's a study project of mine for studying incremental compilers and
package-managers.

The project is still being developed, and the table of contents is:

- [Garbage Collection](#garbage-collection)
- [Macros](#macros)
- [License](#license)

The objective of this project is:

- Dealing with language-servers, and direct support for [Visual Studio Code](https://code.visualstudio.com/)
- Having a full-featured CLI for building, and packaging libraries for Asena
- Lowering to [LLVM-IR](https://llvm.org/), and executing JIT
- Error-resistant compiler, like having a compile-time error, and still can compile, because the error fall onto the
  runtime
- Interpreter for basic things like macro-expanding.
- Query-based architecture.

It's all based on the articles:

- [Resilient parsing](https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html)
- [Typing Haskell in Haskell](http://web.cecs.pdx.edu/~mpj/thih/thih.pdf)
- [Sixty](https://github.com/ollef/sixty)
- [Query-based compiler architetures](https://ollef.github.io/blog/posts/query-based-compilers.html)
- [Rust](https://github.com/rust-lang/rust)
- [Crafting Interpreters](https://craftinginterpreters.com)

And have some great inspirations in [rust-analyzer](https://github.com/rust-lang/rust-analyzer) code-base..

### Garbage-Collection

The goal of this compiler isn't optimizing things, it's more like an IDE, so the garbage collector is based on Reference
Counting.

### Macros

This compiler haves a giant base in procedural macros for generating stubs and trees.

- [asena-derive](asena-derive)
- [asena-hir-derive](asena-hir-derive)

### License

Asena is distributed under the terms of the [MIT license](LICENSE).
