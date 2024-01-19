#!/usr/bin/env bash
# Copyright (c) The Amphitheatre Authors. All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

set -euo pipefail

GOOS="linux" go build -ldflags='-s -w' -o bin/main github.com/amp-buildpacks/{{ packer_name }}/cmd/main

if [ "${STRIP:-false}" != "false" ]; then
  strip bin/main
fi

if [ "${COMPRESS:-none}" != "none" ]; then
  $COMPRESS bin/main
fi

ln -fs main bin/build
ln -fs main bin/detect
