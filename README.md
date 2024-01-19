# packer

Packer generates buildpack with just what you need to start quickly!

## Getting Started

### Install packer

```bash
git clone https://github.com/amp-buildpacks/packer.git
cd packer
cargo install --path packer
```

### Create A New Buildpack

```bash
packer init <buildpack-name>
```

For example:

```bash
packer init solana
```

Or init a empty Buildpack project directory:

```bash
mkdir <buildpack-directory>
cd <buildpack-directory>

packer init
```

### Get More Help

```bash
packer -h
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
