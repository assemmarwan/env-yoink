# env-yoink

env-yoink is a versatile command-line tool written in Rust that simplifies the process of extracting environment
variables from source code. Whether you're working with code written in JavaScript, Go, Python, or any other programming
language, env-yoink offers a convenient and efficient way to identify and collect environment variable references. You
can use it to enhance your development workflow and manage configuration.

## Features

- Regex and Language Presets: env-yoink gives you the flexibility to extract environment variables using regular expressions or predefined language presets.

- Multi-Language Support: With built-in support for popular programming languages like JavaScript, Go, Python, and more.

- Recursive Scanning: Easily scan entire projects or directories recursively, ensuring that no environment variable references are overlooked, even in complex codebases.


## Installation

```bash
cargo install env-yoink
```

## Usage

### Docs

```
env-yoink --help

Tool to grab (yoink) env variables from a workspace into env example file

Usage: env-yoink [OPTIONS] --mode <MODE>

Options:
  -o, --out <OUTPUT_DIRECTORY>                     Output Directory [default: ./]
  -e, --example-file-name <EXAMPLE_FILE_NAME>      Env example file [default: .env.example]
  -d, --workspace-directory <WORKSPACE_DIRECTORY>  Workspace Directory [default: ./]
  -m, --mode <MODE>                                [possible values: regex, preset]
  -r, --regex-pattern <REGEX_PATTERN>
  -p, --preset <PRESET>                            [possible values: js, python, rust, go]
  -h, --help                                       Print help
  -V, --version                                    Print version
```


### Preset Mode

```bash
env-yoink --mode preset --preset js --workspace-directory './src'
```

### Regex Mode

```bash
env-yoink --mode regex --regex-pattern 'process.env.([A-Z_]+)' --workspace-directory './src'
```


## Contributing

Pull requests are welcome and appreciated 😄.

For major changes, please open an issue first to discuss what you would like to change. 

## License

[MIT](https://choosealicense.com/licenses/mit/)