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

type {{ packer_name | capitalize }} struct {
	LayerContributor libpak.DependencyLayerContributor
	Logger           bard.Logger
	Executor         effect.Executor
}

func New{{ packer_name | capitalize }}(dependency libpak.BuildpackDependency, cache libpak.DependencyCache) {{ packer_name | capitalize }} {
	contributor := libpak.NewDependencyLayerContributor(dependency, cache, libcnb.LayerTypes{
		Build:  true,
		Cache:  true,
		Launch: true,
	})
	return {{ packer_name | capitalize }}{
		LayerContributor: contributor,
		Executor:         effect.NewExecutor(),
	}
}

func (r {{ packer_name | capitalize }}) Contribute(layer libcnb.Layer) (libcnb.Layer, error) {
	r.LayerContributor.Logger = r.Logger
	return r.LayerContributor.Contribute(layer, func(artifact *os.File) (libcnb.Layer, error) {
		// TODO: update below codes to your buildpack
		bin := filepath.Join(layer.Path, "bin")

		// TODO: May be use copy instead of it or update Extract Path or stripComponents=1
		r.Logger.Bodyf("Expanding %s to %s", artifact.Name(), bin)
		if err := crush.Extract(artifact, layer.Path, 1); err != nil {
			return libcnb.Layer{}, fmt.Errorf("unable to expand %s\n%w", artifact.Name(), err)
		}

		// Must be set to executable
		// TODO: May be update bin-name
		file := filepath.Join(bin, "{{ packer_name }}")
		r.Logger.Bodyf("Setting %s as executable", file)
		if err := os.Chmod(file, 0755); err != nil {
			return libcnb.Layer{}, fmt.Errorf("unable to chmod %s\n%w", file, err)
		}

		// Must be set to PATH
		r.Logger.Bodyf("Setting %s in PATH", bin)
		if err := os.Setenv("PATH", sherpa.AppendToEnvVar("PATH", ":", bin)); err != nil {
			return libcnb.Layer{}, fmt.Errorf("unable to set $PATH\n%w", err)
		}

		// get {{ packer_name }} version
		buf, err := r.Execute("{{ packer_name }}", []string{"--version"})
		if err != nil {
			return libcnb.Layer{}, fmt.Errorf("unable to get {{ packer_name }} version\n%w", err)
		}
		version := strings.TrimSpace(buf.String())
		r.Logger.Bodyf("Checking {{ packer_name }} version: %s", version)

		// TODO: May be need more codes...
		return layer, nil
	})
}

func (r {{ packer_name | capitalize }}) Execute(command string, args []string) (*bytes.Buffer, error) {
	buf := &bytes.Buffer{}
	if err := r.Executor.Execute(effect.Execution{
		Command: command,
		Args:    args,
		Stdout:  buf,
		Stderr:  buf,
	}); err != nil {
		return buf, fmt.Errorf("%s: %w", buf.String(), err)
	}
	return buf, nil
}


func (r {{ packer_name | capitalize }}) BuildProcessTypes(cr libpak.ConfigurationResolver, app libcnb.Application) ([]libcnb.Process, error) {
	processes := []libcnb.Process{}

	enableDeploy := cr.ResolveBool("BP_ENABLE_{{ packer_name | upper }}_PROCESS")
	if enableDeploy {
		// TODO: Update run command and args
		processes = append(processes, libcnb.Process{
			Type:    "web",
			Command: "<run-command>",
			Arguments: []string{"<command-args>"},
			Default: true,
		})
	}
	return processes, nil
}

func (r {{ packer_name | capitalize }}) Name() string {
	return r.LayerContributor.LayerName()
}
