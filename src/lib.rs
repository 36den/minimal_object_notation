
pub struct MiniON {
    pub name: String,
    pub length: usize,
    pub content: Option<String>,
}

impl MiniON {
    /// Construct a new `MiniON`.
    pub fn new(name: String) -> MiniON {
        MiniON {
            name,
            length: 0,
            content: None,
        }
    }

    /// Set the content of a `MiniON`.
    pub fn set_content(&mut self, content: String) {
        self.length = content.len();
        self.content = Some(content);
    }

    /// Return the `MiniON` as a `String`.
    /// ## Example
    /// ```rust
    ///     use minimal_object_notation::*;
    /// 
    ///     let mut minion = MiniON::new("greeting".to_string());
    /// 
    ///     minion.set_content("Hello, world!".to_string());
    /// 
    ///     let minion = minion.as_string();
    /// ```
    /// Will give you a `String` containing `"greeting|13~Hello, world!"`.
    pub fn as_string(&self) -> String {
        let mut output = String::from(&self.name);
        output.push('|');
        output.push_str(&format!("{}",self.length));
        output.push('~');
        
        match &self.content {
            Some(content) => {
                output.push_str(&content);

                return output;
            },
            None => {
                return output;
            }
        } 
    }

    /// Parse data into a `MiniON` object.
    /// ## Example
    /// ```rust
    ///     use minimal_object_notation::*;
    /// 
    ///     let data = b"greeting|13~Hello, world!";
    /// 
    ///     let mut incr: usize = 0;
    /// 
    ///     match MiniON::parse_one(data, &mut incr) {
    ///         Ok(minion) => {
    ///             assert_eq!("greeting",minion.name);
    ///             
    ///             match minion.content {
    ///                 Some(content) => {
    ///                     assert_eq!("Hello, world!",content);
    ///                 },
    ///                 None => {
    ///                     panic!("Expected content!");
    ///                 }
    ///             }
    ///         },
    ///         Err(e) => {
    ///             panic!("{}",e.to_string());
    ///         }
    ///     }
    /// 
    /// ```
    pub fn parse_one(bytes: &[u8], incr: &mut usize) -> Result<MiniON,Error> {
        let mut name = String::new();
        let mut length: usize = 0;
        let mut content = String::new();

        match MiniON::parse_name(bytes, incr) {
            Ok(n) => {
                name = n;
            },
            Err(e) => {
                return Err(e);
            }
        }

        match MiniON::parse_length(bytes, incr, &name) {
            Ok(n) => {
                length = n;
            },
            Err(e) => {
                return Err(e);
            }
        }

        if length != 0 {
            match MiniON::parse_content(bytes, incr, &name, length) {
                Ok(n) => {
                    content = n;
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }

        let mut minion = MiniON::new(name);
        if length != 0 {
            minion.set_content(content);
        }

        return Ok(minion);
    }

    /// Parse data that contains multiple miniON objects ONE AFTER THE OTHER. Will not parse nested miniON objects.
    /// ## Example
    /// ```rust
    ///     use minimal_object_notation::*;
    /// 
    ///     let data = b"first|4~ONE,second|4~TWO,third|6~THREE,container|29~name|5~NAME,content|7~CONTENT";
    /// 
    ///     match MiniON::parse_all(data) {
    ///         Ok(minions) => {
    ///             assert_eq!(4,minions.len());
    /// 
    ///             assert_eq!("first",minions[0].name);
    /// 
    ///             assert_eq!("second",minions[1].name);
    /// 
    ///             assert_eq!("third",minions[2].name);
    /// 
    ///             assert_eq!("container",minions[3].name);
    /// 
    ///             match &minions[0].content {
    ///                 Some(content) => {
    ///                     assert_eq!("ONE,",content);
    ///                 },
    ///                 None => {
    ///                     panic!("Expected content!");
    ///                 }
    ///             }
    /// 
    ///             match &minions[3].content {
    ///                 Some(content) => {
    ///                     assert_eq!("name|5~NAME,content|7~CONTENT",content);
    ///                 },
    ///                 None => {
    ///                     panic!("Expected content!");
    ///                 }
    ///             }
    ///         },
    ///         Err(e) => {
    ///             panic!("{}",e.to_string());
    ///         }
    ///     }
    /// ```
    pub fn parse_all(bytes: &[u8]) -> Result<Vec<MiniON>,Error> {

        let mut minions: Vec<MiniON> = Vec::new();

        let mut incr: usize = 0;

        loop {
            match MiniON::parse_one(bytes, &mut incr) {
                Ok(minion) => {
                    minions.push(minion);

                    if incr == bytes.len() {
                        return Ok(minions);
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    /// Parse the name of a miniON object. (Start at the correct position.)
    /// ## Example
    /// ```rust
    ///     use minimal_object_notation::*;
    /// 
    ///     let data = b"greeting|13~Hello, world!";
    /// 
    ///     let mut incr: usize = 0;
    /// 
    ///     match MiniON::parse_name(data,&mut incr) {
    ///         Ok(name) => {
    ///             assert_eq!("greeting",name);
    ///             assert_eq!(9,incr);
    ///         },
    ///         Err(e) => {
    ///             panic!("{}",e.to_string());
    ///         }
    ///     }
    /// ```
    pub fn parse_name(bytes: &[u8], incr: &mut usize) -> Result<String,Error> {
        let mut output = String::new();
    
        loop {
    
            match bytes[*incr] as char {
                '|' => {
                    match *incr + 1 < bytes.len() {
                        true => {
                            *incr += 1;
                        },
                        false => {
                            return Err(Error::Incomplete(format!("No more data after name ({}) field at position {}.",output,*incr)));
                        }
                    }
    
                    return Ok(output);
                },
                c => {
                    output.push(c);
                }
            }
    
            match *incr + 1 < bytes.len() {
                true => {
                    *incr += 1;
                },
                false => {
                    return Err(Error::NoStructure);
                }
            }
    
        }
    }

    /// Parse the length of a miniON object (after having parsed the name tag).
    /// ## Example
    /// ```rust
    ///     use minimal_object_notation::*;
    /// 
    ///     let data = b"greeting|13~Hello, world!";
    /// 
    ///     let mut incr: usize = 9;
    /// 
    ///     match MiniON::parse_length(data,&mut incr,"greeting") {
    ///         Ok(length) => {
    ///             assert_eq!(13,length);
    ///             assert_eq!(12,incr);
    ///         },
    ///         Err(e) => {
    ///             panic!("{}",e.to_string());
    ///         }
    ///     }
    /// ```
    pub fn parse_length(bytes: &[u8], incr: &mut usize, name: &str) -> Result<usize,Error> {
        let mut output = String::new();
    
        loop {
    
            match bytes[*incr] as char {
                '~' => {
                    match *incr + 1 < bytes.len() {
                        true => {
                            *incr += 1;
                        },
                        false => {
                            match output.parse::<usize>() {
                                Ok(length) => {
                                    match length == 0 {
                                        true => {
                                            return Ok(length);
                                        },
                                        false => {
                                            return Err(Error::Incomplete(format!("No more data after length (name: {}) field at position {}.",name,*incr)));
                                        }
                                    }
                                },
                                Err(_) => {
                                    return Err(Error::BadStructure(format!("Could not parse the length field. Contains: {}",output)));
                                }
                            }
                        }
                    }
    
                    match output.parse::<usize>() {
                        Ok(length) => {
                            return Ok(length);
                        },
                        Err(_) => {
                            return Err(Error::BadStructure(format!("Could not parse the length field. Contains: {}",output)));
                        }
                    }
    
                },
                c => {
                    output.push(c);
                }
            }
    
            match *incr + 1 < bytes.len() {
                true => {
                    *incr += 1;
                },
                false => {
                    return Err(Error::NoStructure);
                }
            }
    
        }
    }

    /// Parse the contents of a miniON object (after having parsed the name and length tags).
    /// ## Example
    /// ```rust
    ///     use minimal_object_notation::*;
    /// 
    ///     let data = b"greeting|13~Hello, world!";
    /// 
    ///     let mut incr: usize = 12;
    /// 
    ///     match MiniON::parse_content(data, &mut incr, "greeting", 13) {
    ///         Ok(content) => {
    ///             assert_eq!("Hello, world!",content);
    ///             assert_eq!(incr,data.len());
    ///         },
    ///         Err(e) => {
    ///             panic!("{}",e.to_string());
    ///         }
    ///     }
    /// ```
    /// ## Warning!
    /// Should not be called when the object has a length of 0! This will result in errors!
    pub fn parse_content(bytes: &[u8], incr: &mut usize, name: &str, length: usize) -> Result<String,Error> {
        let mut output = String::new();
    
        let mut pos_count: usize = 0;
    
        loop {
    
            pos_count += 1;
    
            output.push(bytes[*incr] as char);
    
            match *incr + 1 < bytes.len() {
                true => {
                    match pos_count < length {
                        true => {
                            *incr += 1;
                        },
                        false => {
                            *incr += 1;
    
                            return Ok(output);
                        }
                    }
                },
                false => {
    
                    match pos_count == length {
                        true => {
                            *incr += 1;

                            return Ok(output);
                        },
                        false => {
                            return Err(Error::Incomplete(format!("The object (name: {}) is incomplete. Bytes missing = {} .",name, length - pos_count )));
                        }
                    }
                    
                }
            }
    
        }

    }
    
}

pub enum Error {
    Incomplete(String),
    NoStructure,
    BadStructure(String),
    NoContent,
}

impl Error {
    /// Will `println!` the error with an explanation for you. 
    pub fn print(&self) {
        match self {
            Error::Incomplete(info) => {
                println!("Error: Incomplete data: {}",info);
            },
            Error::NoStructure => {
                println!("Error: No structure: The data does not follow the mON structure.")
            },
            Error::BadStructure(info) => {
                println!("Error: Bad data: {}",info);
            },
            Error::NoContent => {
                println!("Error: Content of length 0 cannot be parsed.")
            }
        }
    }

    /// Will give you a `String` with the relevant info.
    pub fn to_string(&self) -> String {
        match self {
            Error::Incomplete(info) => {
                return format!("Error: Incomplete data: {}",info);
            },
            Error::NoStructure => {
                return format!("Error: No structure: The data does not follow the mON structure.")
            },
            Error::BadStructure(info) => {
                return format!("Error: Bad data: {}",info);
            },
            Error::NoContent => {
                return format!("Error: Content of length 0 cannot be parsed.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_minion() {
        let mut minion = MiniON::new("greeting".to_string());

        minion.set_content("Hello, world!".to_string());

        assert_eq!("greeting|13~Hello, world!",minion.as_string());

    }

    #[test]
    fn test_multi_minion() {
        
        let mut minion_container = MiniON::new("container".to_string());

        let mut days_of_the_week = MiniON::new("object".to_string());

        days_of_the_week.set_content("____________________".to_string());

        let mut pairs_of_socks = MiniON::new("object".to_string());

        pairs_of_socks.set_content("____________________".to_string());

        let mut content = days_of_the_week.as_string();
        content.push_str(&pairs_of_socks.as_string());

        minion_container.set_content(content);

        assert_eq!("container|60~object|20~____________________object|20~____________________",minion_container.as_string());
    }

    #[test]
    fn test_parse_manually() {
        let data = b"greeting|13~Hello, world!name|6~miniON";

        let mut incr: usize = 0;

        match MiniON::parse_name(data, &mut incr) {
            Ok(name) => {
                assert_eq!("greeting",name);
            },
            Err(e) => {
                panic!("{}",e.to_string());
            }
        }

        match MiniON::parse_length(data, &mut incr, "greeting") {
            Ok(length) => {
                assert_eq!(13,length);
            },
            Err(e) => {
                panic!("{}",e.to_string());
            }
        }

        match MiniON::parse_content(data, &mut incr, "greeting", 13) {
            Ok(content) => {
                assert_eq!("Hello, world!",content);
            },
            Err(e) => {
                panic!("{}",e.to_string());
            }
        }

        match MiniON::parse_name(data, &mut incr) {
            Ok(name) => {
                assert_eq!("name",name);
            },
            Err(e) => {
                panic!("{}",e.to_string());
            }
        }

        match MiniON::parse_length(data, &mut incr, "name") {
            Ok(length) => {
                assert_eq!(6,length);
            },
            Err(e) => {
                panic!("{}",e.to_string());
            }
        }

        match MiniON::parse_content(data, &mut incr, "name", 6) {
            Ok(content) => {
                assert_eq!("miniON",content);
            },
            Err(e) => {
                panic!("{}",e.to_string());
            }
        }
    }

    #[test]
    fn test_parse_one() {
        let data = b"greeting|13~Hello, world!";

        let mut incr: usize = 0;

        match MiniON::parse_one(data, &mut incr) {
            Ok(minion) => {
                assert_eq!("greeting",minion.name);
                
                match minion.content {
                    Some(content) => {
                        assert_eq!("Hello, world!",content);
                    },
                    None => {
                        panic!("No content!");
                    }
                }
            },
            Err(e) => {
                panic!("{}",e.to_string());
            }
        }
    }

    #[test]
    fn test_parse_all() {
        let data = b"title|12~grocery listdate|10~04/08/2020grocery list|21~1.|6~cheese2.|5~bread";

        match MiniON::parse_all(data) {
            Ok(minions) => {
                assert_eq!(3,minions.len());

                assert_eq!("title",minions[0].name);
                
                match &minions[0].content {
                    Some(content) => {
                        assert_eq!("grocery list",content);
                    },
                    None => {
                        panic!("Expected content!");
                    }
                }

                assert_eq!("date",minions[1].name);

                match &minions[1].content {
                    Some(content) => {
                        assert_eq!("04/08/2020",content);
                    },
                    None => {
                        panic!("Expected content!");
                    }
                }

                assert_eq!("grocery list",minions[2].name);

                match &minions[2].content {
                    Some(content) => {

                        assert_eq!("1.|6~cheese2.|5~bread",content);
                        
                        match MiniON::parse_all(content.as_bytes()) {
                            Ok(minions) => {
                                assert_eq!(2,minions.len());

                                assert_eq!("1.",minions[0].name);

                                match &minions[0].content {
                                    Some(content) => {
                                        assert_eq!("cheese",content);
                                    },
                                    None => {
                                        panic!("Expected content!");
                                    }
                                }

                                assert_eq!("2.",minions[1].name);

                                match &minions[1].content {
                                    Some(content) => {
                                        assert_eq!("bread",content);
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
    }
}
