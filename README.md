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

## HTTP server

    curl localhost:8080/jsonapi/model/1  | curl -v -H 'Content-Type: application/json' -X PUT --data-binary @- http://localhost:8080/jsonapi/model/1

    curl localhost:8080/jsonapi/model/1  | jq '.body.xflows[1]'

## Docker

    docker run -p 8080:8080 --expose 8080 -v "/tmp/x3:/project" -e RUST_LOG=info gearsproject/gears-cli:latest


