_default:
    @just --list

gen-types:
    typeshare ./server --lang=typescript --output-file=web-client/src/shared_definitions.ts

# Top-level dev command. Not recommended for development, just for ease of use.
dev:
    just web-client/ & just server/
