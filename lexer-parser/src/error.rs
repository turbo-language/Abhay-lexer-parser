use std::fmt;
use std::io;



// Types

#[derive(Debug)]
pub struct Error {
    //pub kind: ErrorKind,
    pub line_num: usize,
}


