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

* [ ] **Lexer** (in-progress)
* [ ] Parser (Recursive Descent, LL(1))
* [ ] Code Generator
* [ ] Semantic Checker
* [ ] Garbage Collection
* [ ] JIT Compiler