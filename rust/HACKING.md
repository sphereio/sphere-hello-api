### on mac:

```
brew install fswatch

fswatch -o src -o examples | xargs -n1 cargo test
```
