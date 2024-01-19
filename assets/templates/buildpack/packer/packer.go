// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package {{ packer_name }}

import (
	"bytes"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/buildpacks/libcnb"
	"github.com/paketo-buildpacks/libpak"
	"github.com/paketo-buildpacks/libpak/bard"
	"github.com/paketo-buildpacks/libpak/crush"
	"github.com/paketo-buildpacks/libpak/effect"
	"github.com/paketo-buildpacks/libpak/sherpa"
)

type CustomLayer struct {
	LayerContributor libpak.DependencyLayerContributor
	Logger           bard.Logger
	Executor         effect.Executor
}

func NewCustomLayer(dependency libpak.BuildpackDependency, cache libpak.DependencyCache) CustomLayer {
	contributor := libpak.NewDependencyLayerContributor(dependency, cache, libcnb.LayerTypes{
		Build:  true,
		Cache:  true,
		Launch: true,
	})
	return CustomLayer{
		LayerContributor: contributor,
		Executor:         effect.NewExecutor(),
	}
}

func (c CustomLayer) Contribute(layer libcnb.Layer) (libcnb.Layer, error) {
	c.LayerContributor.Logger = c.Logger
	return c.LayerContributor.Contribute(layer, func(artifact *os.File) (libcnb.Layer, error) {
		// TODO: update below codes to your buildpack
		bin := filepath.Join(layer.Path, "bin")

		// TODO: May be use copy instead of it.
		c.Logger.Bodyf("Expanding %s to %s", artifact.Name(), bin)
		if err := crush.Extract(artifact, layer.Path, 1); err != nil {
			return libcnb.Layer{}, fmt.Errorf("unable to expand %s\n%w", artifact.Name(), err)
		}

		// Must be set to PATH
		c.Logger.Bodyf("Setting %s in PATH", bin)
		if err := os.Setenv("PATH", sherpa.AppendToEnvVar("PATH", ":", bin)); err != nil {
			return libcnb.Layer{}, fmt.Errorf("unable to set $PATH\n%w", err)
		}

		// get {{ packer_name }} version
		buf, err := c.Execute("{{ packer_name }}", []string{"--version"})
		if err != nil {
			return libcnb.Layer{}, fmt.Errorf("unable to get {{ packer_name }} version\n%w", err)
		}
		version := strings.TrimSpace(buf.String())
		c.Logger.Bodyf("Checking {{ packer_name }} version: %s", version)

		// TODO: May be need more codes...
		return layer, nil
	})
}

func (c CustomLayer) Execute(command string, args []string) (*bytes.Buffer, error) {
	buf := &bytes.Buffer{}
	if err := c.Executor.Execute(effect.Execution{
		Command: command,
		Args:    args,
		Stdout:  buf,
		Stderr:  buf,
	}); err != nil {
		return buf, fmt.Errorf("%s: %w", buf.String(), err)
	}
	return buf, nil
}

func (c CustomLayer) Name() string {
	return c.LayerContributor.LayerName()
}
