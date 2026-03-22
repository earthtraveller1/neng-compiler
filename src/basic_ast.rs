// For now, I'm only going to implement the ones that are trivial to implement
// because i am lazy lol
pub enum BasicAST<'a> {
    Init(&'a str, u8),
    Set(&'a str, u8),
    Increment(&'a str),
    Decrement(&'a str),
    For(&'a str, Block<'a>),    
    Function(&'a str, Vec<&'a str>, Block<'a>),
    CallFunction(&'a str, Vec<&'a str>)
}

pub struct Variable<'a> {
    name: &'a str,
    offset: usize,
}

pub struct Block<'a> {
    local_vars: Vec<Variable<'a>>,
    body: Vec<BasicAST<'a>>
}

// Oh god the lifetimes are spreading everywhere
