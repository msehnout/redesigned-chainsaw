#!/bin/bash

rustdoc --no-defaults --passes collapse-docs --passes unindent-comments --passes strip-priv-imports src/main.rs 
