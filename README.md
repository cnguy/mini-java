## Mini-Java implements a subset of Java, in Rust.

```
class Test {
    void main() {
        writeInt(factorial(12));
    }

    int factorial(int i) {
        int p;
        p = 1;
        while (i > 1) {
            p = p * i;
            i = i - 1;
        }
        return p;
    }
}
```

## Progress:

[x] does not exactly mean done. It just means I consider it done. There may still be bugs (e.g. no overflow is accounted for in Lexer right now)

* [x] Lexer
* [ ] **Parser (Recursive Descent, LL(1))** (in-progress)
* [ ] Code Generator
* [ ] Semantic Checker
* [ ] Garbage Collection
* [ ] JIT Compiler