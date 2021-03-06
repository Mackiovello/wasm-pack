# pack and publish 

The `publish` and `pack` commands interact with the pkg directory that's 
created when you run `wasm-pack init`. The `pack` command creates a tarball
from the pkg directory and the `publish` command creates a tarball from the 
pkg directory __and__ publishes it to the NPM registry. 

Underneath, these commands use `npm pack` and `npm publish`. You can read
more about these in the NPM documentation:

- [`npm pack`](https://docs.npmjs.com/cli/pack)
- [`npm publish`](https://docs.npmjs.com/cli/publish)

Both these commands take the path to the pkg directory as the first argument. 
You can either set the argument directly to the pkg directory or to the parent 
of the pkg directory:

```
$ wasm-pack pack myproject/pkg
| 🎒  packed up your package!
$ wasm-pack pack myproject
| 🎒  packed up your package!
```

If you try to call `pack` or `publish` on another directory, you get an error:

```
$ wasm-pack pack myproject/src/
Unable to find the pkg directory at path 'myproject/src/', or in a child directory of 'myproject/src/'
```

If you don't set a path, they use the current directory as the path.