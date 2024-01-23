# packer

Packer generates buildpack with just what you need to start quickly!

## Getting Started

### Install packer

```bash
git clone https://github.com/amp-buildpacks/packer.git
cd packer
cargo install --path packer

## Or
cargo install --git https://github.com/amp-buildpacks/packer.git
```

### Create A New Buildpack

```bash
packer init <buildpack-name>
```

For example:

```bash
packer init solana
```

Or Initialize an empty Buildpack project directory:

```bash
mkdir <buildpack-directory>
cd <buildpack-directory>

packer init
```

### Get More Help

```bash
packer -h
```

## Configuration File Supports Variables

### Buildpack Template

```toml
[[dependencies]]
id = "leo-gnu"
name = "Leo (GNU libc)"
pkg_name = "leo"
sha256 = "abcd29454e940dd320b6915569f840a9ffe2515045c06667b5aa2ad34f7e0320"
uri = "https://github.com/AleoHQ/leo/releases/download/v1.10.0/leo-v1.10.0-x86_64-unknown-linux-gnu.zip"
version = "1.10.0"
license = "GNU"

[[dependencies]]
id = "leo-musl"
name = "Leo (musl libc)"
pkg_name = "leo"
sha256 = "508264f03760d0a0c9d8cd13c603e0e0d595388b3729762ebfbcc26abe46d667"
uri = "https://github.com/AleoHQ/leo/releases/download/v1.10.0/leo-v1.10.0-x86_64-unknown-linux-musl.zip"
version = "1.10.0"
license = "GNU"
```

### Meta Template

```toml
[[dependencies]]
repo = "amp-buildpacks/leo"
version = "0.1.2"
```

### Builder Template

```toml
[[dependencies]]
repo = "amp-buildpacks/leo-dist"
version = "0.1.2"

[[dependencies]]
repo = "amp-buildpacks/aleo"
version = "0.1.2"
```

## Contributing

If anything feels off, or if you feel that some functionality is missing, please
check out the [contributing
page](https://docs.amphitheatre.app/contributing/). There you will find
instructions for sharing your feedback, building the tool locally, and
submitting pull requests to the project.

## License

Copyright (c) The Amphitheatre Authors. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

      https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## Credits

Heavily inspired by https://github.com/foundry-rs/foundry
