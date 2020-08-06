# Minimal Object Notation (aka. MiniON)

A Rust crate for working with `miniON`s.

## Example creating `miniON`s

```rust
    use minimal_object_notation::*;

    let mut minion = MiniON::new("greeting".to_string());

    minion.set_content("Hello, world!".to_string());

    let minion = minion.to_string();
```

## Example parsing `miniON`s

```rust
    use minimal_object_notation::*;

    let data = b"greeting|13~Hello, world!container|23~first|3~ONEsecond|3~TWO";

    let mut incr: usize = 0;

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

    // OR

    match Minion::parse_all(data) {
        Ok(minions) => {
            assert_eq!(minions.len(),2);

            assert_eq!("container",minions[1].name);
        },
        Err(e) => {
            panic!("{}",e.to_string());
        }
    }
```
