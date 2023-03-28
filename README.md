# Glossary [![Github](https://github.com/waltonseymour/glossary/workflows/Rust/badge.svg)](https://github.com/waltonseymour/glossary/actions)

A flat file indexer in rust

### Install
`cargo install glossary`

### Basic Usage
To index the first column of the csv:

`glossary index data.csv`

After the index has been generated:

`glossary find data.csv user@email.com`


### Alternative impls of index format

* [`Python`](https://github.com/KOLANICH-libs/glossary.py)
* [`Kaitai Struct`](https://github.com/KOLANICH-specs/kaitai_struct_formats/blob/glossary_index/database/glossary_index.ksy)
