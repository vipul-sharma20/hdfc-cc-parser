# hdfc-cc-parser

Python package to parse HDFC credit card statement to a CSV. This is a Python
package for [hdfc-cc-parser-rs][hdfc-cc-parser-rs].

> [!NOTE]
> Read original project's README for details on the CLI tool. Limiting this
> README to Python package only

### Installation

```
pip install https://github.com/vipul-sharma20/hdfc-cc-parser/releases/download/v0.0.6/hdfc_cc_parser_rs-0.0.6-cp310-cp310-macosx_11_0_arm64.whl
```

### Build Package Manually

> [!NOTE]
> You may want to build Python wheels and install manually until I start
> publishing universal wheels. Currently, I have whl that should support MacOS
> arm64

1. Clone the repo and follow CLI installation instructions in [hdfc-cc-parser-rs][hdfc-cc-parser-rs].
2. Copy the binary `./target/release/hdfc-cc-parser-rs` to `/usr/bin/local/hdfc-cc-parser-rs`
3. Install `maturin` by `pip install maturin`
4. Build using `maturin build --release`
5. If everything goes well, you should be able to import the Python package as `import hdfc_cc_parser`

## Usage

```python
import hdfc_cc_parser

response = hdfc_cc_parser.parse_cc_statement(file_path, name, password)
```

[hdfc-cc-parser-rs]: https://github.com/joeirimpan/hdfc-cc-parser-rs
