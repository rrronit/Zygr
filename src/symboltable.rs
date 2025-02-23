use std::collections::HashMap;

use crate::parser::Node;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub type_annotation: Option<Node>,
    pub scope: Scope,
    pub is_constant: bool,
}

#[derive(Debug, Clone)]
pub enum Scope {
    Global,
    Function,
    Block,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent,
        }
    }

    pub fn insert(&mut self, symbol: Symbol) -> Result<(), String> {
        if self.symbols.contains_key(&symbol.name) {
            return Err(format!("Symbol '{}' already exists", symbol.name));
        }
        self.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name).or_else(|| {
            self.parent.as_ref().and_then(|parent| parent.lookup(name))
        })
    }
}