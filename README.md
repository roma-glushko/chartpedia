# 📚 chartpedia

A convenient way to document your Helm charts using short metadata in a values.yaml file:

```yaml
## @section Image Details
## @descriptionStart
## Redis Image Configuration
## Ref: https://hub.docker.com/_/redis
## @descriptionEnd
##
image:
  ## @param image.repository [default: redis] Redis image repistory
  repository: redis
  ## @param image.tag Redis image tag or a digest (in a form of sha256:aa..)
  tag: 7.2.4
  ## @param image.pullPolicy Image pull policy
  pullPolicy: IfNotPresent
  ## @param image.pullSecrets Specify docker-registry secret names as an array
  pullSecrets: []
```

## Features

- 📦 Easy to install on MacOS, Linux, and Windows. Everything is packed into one slim binary.
- ✅ Fully compatible with the original [Bitnami Readme Generator](https://github.com/bitnami/readme-generator-for-helm/) format. If you are already using it, no change is needed to your values.yaml metadata to get started.
- 🛠️ Support JSON and YAML configs
- 👷 CI-friendly `check` command
- 🦀 Written in blazing-fast Rust

## Installation

Coming soon

## Usage

Coming soon

# Credits

The idea is taken from [the Bitnami Readme Generator](https://github.com/bitnami/readme-generator-for-helm/)

Made with ❤️ by [Roma Hlushko](https://github.com/roma-glushko), Apache-2.0.
