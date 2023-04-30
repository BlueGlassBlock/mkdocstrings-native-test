# mkdocstrings-native-test(async-trash)

Steps (assuming you have PDM installed):

```sh
pdm install --no-self # Skip PEP 517 build process
pdm run mkdocs serve # Intended documentaion
pdm run maturin develop # Build the native Rust extension
pdm run mkdocs serve # A mess
```