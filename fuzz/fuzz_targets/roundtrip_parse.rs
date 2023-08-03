#![no_main]

use libfuzzer_sys::{fuzz_target, Corpus};

fuzz_target!(|case: &[u8]|  -> Corpus  {
    
    do_fuzz(case)
});

use std::rc::Rc;

use anyhow::{Error, Result};
use semver::Version;

use slang_solidity::{
    language::Language,
    syntax::nodes::{
        Cursor, ProductionKind, Visitor, VisitorExitResponse, TokenNode
    },
};

struct Source {
    source_code: String,
}

impl Visitor<Error> for Source {
    fn token(&mut self, node: &Rc<TokenNode>, _cursor: &Cursor) -> Result<VisitorExitResponse, Error> {
        self.source_code += &node.text;
        Ok(VisitorExitResponse::Continue)
    }
}


fn do_fuzz(case: &[u8]) -> Corpus {
    let Ok(code) = std::str::from_utf8(case) else { return Corpus::Reject; };
    let language = Language::new(Version::parse("0.8.19").unwrap()).unwrap();
    let parse_output = language.parse(ProductionKind::ContractDefinition, &code).unwrap();

    let mut collector = Source {
        source_code: String::new(),
    };

    parse_output
        .parse_tree()
        .cursor()
        .drive_visitor(&mut collector).unwrap();


    // The unparsed syntax tree should be the same as the original source code.
    assert_eq!(collector.source_code.as_str(), code);

    Corpus::Keep
}