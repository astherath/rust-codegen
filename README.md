# Rust-Codegen

## Project Goals (Priority List)

1. Read a user-created config/schema file (toml/json) and automatically generate API endpoints using the `actix` web framework

2. Generated code should be completely bug free (by design)

3. Generated code must be able to include extension opportunities if requested explicitly


## Development Roadmap

#### Stage One - Architecture and design choices

- frameworks/dependencies
- file I/O type (probably toml with json extensions available)
- file hierarchy and structure

#### Stage Two - User facing interfaces & input file parser

- File parser development

- CLI tool creation (using `clap`)

#### Stage Three - Codegen lib development

- Codegen lib structure and design

- Codegen prototyping and tooling

#### Stage Four - Testing, build, and deployment

- Unit test suite

- E2E component test suite
