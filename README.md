# Glossary [![Github](https://github.com/waltonseymour/glossary/workflows/Rust/badge.svg)](https://github.com/waltonseymour/glossary/actions)

A flat file indexer in rust

### Install
`cargo install glossary`

### Basic Usage
To index the first column of the csv:

`glossary index data.csv`

After the index has been generated:

`glossary find user@email.com`
