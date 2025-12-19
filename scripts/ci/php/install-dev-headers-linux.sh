#!/usr/bin/env bash
set -euo pipefail

sudo apt-get update
PHP_VERSION="$(php -r 'echo PHP_MAJOR_VERSION.".".PHP_MINOR_VERSION;')"

# `php-dev` / `php${PHP_VERSION}-dev` depends on having a PHP SAPI installed. When PHP is
# provided by `shivammathur/setup-php`, APT may not see `php${PHP_VERSION}-cli` as installed
# and will choose a different provider (often `php${PHP_VERSION}-litespeed`), which currently
# fails its post-install script on GitHub's Ubuntu runners. Installing the CLI package first
# ensures the dependency is satisfied without pulling litespeed.
sudo apt-get install -y --no-install-recommends "php${PHP_VERSION}-cli" "php${PHP_VERSION}-dev"
