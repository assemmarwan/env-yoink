# env-yoink

`env-yoink` is a versatile command-line tool written in Rust that simplifies the process of extracting environment variables from source code. Whether you're working with code written in JavaScript, Go, Python, or any other programming language, `env-yoink` offers a convenient way to identify and collect environment variable references. You can use it to enhance your development workflow and manage configuration.

## Features

- Regex and Language Presets: `env-yoink` gives you the flexibility to extract environment variables using regular expressions or predefined language presets.

- Multi-Language Support: With built-in support for popular programming languages like JavaScript, Go, Python, and more.

- Recursive Scanning: Using the amazing [ripgrep](https://github.com/BurntSushi/ripgrep), easily scan entire projects or directories recursively, while respecting `.gitignore`d files ensuring that no environment variable references are overlooked, even in complex codebases.

## Installation

As of now, the best way to install `env-yoink` is through `cargo` command. Make sure to have Rust installed on your machine and then run:

```bash
cargo install env-yoink
```
### Other Package Managers

A platform specific installation method will be available soon across platforms (Homebrew, pacman, APT, etc...)

_Coming Soon..._

## Usage

### Docs

```
env-yoink --help

Tool to grab (yoink) env variables from a workspace into env example file

Usage: env-yoink [OPTIONS] <WORKSPACE_DIRECTORY>

Arguments:
  <WORKSPACE_DIRECTORY>  Workspace Directory

Options:
  -o, --out <OUTPUT_DIRECTORY>
          Output Directory [default: ./]
  -e, --example-file-name <EXAMPLE_FILE_NAME>
          Env example file [default: .env.example]
  -x, --regex-pattern <REGEX_PATTERN>
          Custom Regex pattern to capture the env variables
  -p, --preset <PRESET>
          Use from the list of regex presets based on the language [possible values: js, python, rust, go]
  -h, --help
          Print help
  -V, --version
          Print version

```

### Preset Mode

```bash
env-yoink './src' --preset js
```

### Regex Mode

```bash
env-yoink './src' --regex-pattern 'process.env.([A-Z_]+)'
```

## Examples

Suppose we have this file in our project directory.

```typescript
// my-project/config.ts

// Get the env variables 
const API_URL = process.env.API_URL;
const SECRET_KEY = process.env['SECRET_KEY']; // This incosistency is intentional and for demo purposes only, relax :)

// Initialize the client
const client = new Client(API_URL, SECRET_KEY);

// Call the API...
```

While in the project directory, run:
```bash
env-yoink . --preset=js
```

This will write to the file `.env.example` (by default) as show below:

```bash
# .env.example

API_URL=
SECRET_KEY=
```

## Roadmap
- [ ] More Presets (Deno, Java, C#, etc...)
- [ ] Offer the CLI tool through different package managers
- [ ] Differentiate between commented out code
- [ ] Append `.env.example` instead of overwriting it
- [ ] Add option to dry-run
- [ ] Verbose Mode
- [ ] Write tests
- [ ] Extend functionality to provide linting using [dotenv-linter](https://github.com/dotenv-linter/dotenv-linter)
- [ ] Optimise Performance
- [ ] More...

## Contributing
Pull requests are welcome and appreciated 😄.

For major changes, please open an issue first to discuss what you would like to change. 
## License

[MIT](https://choosealicense.com/licenses/mit/)
