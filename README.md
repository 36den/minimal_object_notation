# Minimal Object Notation

A Rust crate for reading and writing minimimal object notation.

## Introduction

What is 'Minimal Object Notation'? It is a format that comes from simply attaching a name tag and a length tag to some information. For example: `greeting|13~Hello, world!`.

## Example creating `miniON`s

```rust
    use minimal_object_notation::*;

    let mut minion = MiniON::new("greeting".to_string());

    minion.set_content("Hello, world!".to_string());

    let minion = minion.to_string();
```
Will result in a `String` containing `greeting|13~Hello, world!`.

## Example parsing `miniON`s

```rust
    use minimal_object_notation::*;

    let data = b"greeting|13~Hello, world!container|23~first|3~ONEsecond|3~TWO";

    let mut incr: usize = 0;

    // Parse a single object that starts at the position `incr`...

    match MiniON::parse_one(data, &mut incr) {
        Ok(minion) => {
            assert_eq!("greeting",minion.name);

            match minion.content {
                Some(content) => {
                    assert_eq!("Hello, world!",content);
                },
                None => {
                    panic!("Expected content!");
                }
            }
        },
        Err(e) => {
            panic!("{}",e.to_string());
        }
    }

    // ... OR parse all (sucessive) miniON objects.

    match MiniON::parse_all(data) {
        Ok(minions) => {
            assert_eq!(minions.len(),2);

            assert_eq!("container",minions[1].name);
        },
        Err(e) => {
            panic!("{}",e.to_string());
        }
    }
```
