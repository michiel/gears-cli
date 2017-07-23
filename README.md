[![Build Status](https://travis-ci.org/gears-project/gears-cli.svg?branch=master)](https://travis-ci.org/gears-project/gears-cli)
[![Build status](https://ci.appveyor.com/api/projects/status/e6k86vca03kglpcg/branch/master?svg=true)](https://ci.appveyor.com/project/michiel/gears-cli/branch/master)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/gears-project/gears-core-rust/master/LICENSE)

# gears-cli

This tool generates projects suitable for version control.

It is recommended to initialize this project with `git`, though any VCS will do.

    git init
    git add .
    git commit -m "Initial commit"

## Help

Have the `gears-cli` tool installed. See 
[http://github.com/gears-project/gears-cli](http://github.com/gears-project/gears-cli)

For general information, visit the project hub at
[http://github.com/gears-project/](http://github.com/gears-project/)

    gears-cli --help

## Build

    gears-cli build

## Usage

    gears-cli export-json | gears-cli import-json --  

### Interactive shell

    gears-cli shell

    << Running gears-shell
    >> list xflow
    XFlow: ID Uuid("606dc85d-9daf-4045-8b85-0c7ccb667c63") - "zork"
    >> generate xflow my_first_xflow
    XFlow: ID Uuid("5e0d1a30-9c48-489c-af2d-a34054c98316") - "my_first_xflow"
    >> generate page my_first_page
    Page: ID Uuid("fc016992-95ad-49aa-9cb4-9814ce803d9a") - "my_first_page"
    >> generate translation es_ES
    >> list translation
    Translation: ID Uuid("0cab532f-3c5c-49a7-89c0-9132e14039a8") - "default" - "en_US"
    Translation: ID Uuid("5f64834b-bfb4-4075-966d-0d8a4cfe6232") - "default" - "es_ES"
    >> sync

When using the interactive shell to make changes, remember that changes are **ONLY SAVED AFTER
ISSUING A `sync` COMMAND**.

