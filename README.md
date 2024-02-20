# chartpedia

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

## Installation

Coming soon

## Usage

Coming soon

# Credits

The idea is taken from [the Bitnami Readme Generator](https://github.com/bitnami/readme-generator-for-helm/)

Made with ❤️ by [Roma Hlushko](https://github.com/roma-glushko), Apache-2.0.
