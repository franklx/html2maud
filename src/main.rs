// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use std::{default::Default, vec};
use std::io;
use std::string::String;

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use rcdom::{Handle, NodeData, RcDom};

fn walk(indent: usize, handle: &Handle) {
    let node = handle;
    match node.data {
        NodeData::Document => {
            println!("html! {{");
            for child in node.children.borrow().iter() {
                walk(indent + 4, child);
            }
            println!("}}");
        },

        NodeData::Doctype {
            name: _,
            public_id: _,
            system_id: _,
        } => println!("{:indent$}(DOCTYPE)", "", indent=indent),

        NodeData::Text { ref contents } => {
            let text = &contents.borrow();
            let text = text.trim();
            if text.len() > 0 {
                println!("{:indent$}\"{}\"", "", escape_default(text), indent=indent);
            }
        },

        NodeData::Comment {
            ref contents
        } => println!("{:indent$}/* {} */", "", contents, indent=indent),

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let mut a_tag = &name.local[..];
            let mut a_cls = vec![""];
            let mut a_id = vec![""];
            let mut a_props= vec![String::new()];
            let attrs = attrs.borrow();
            for attr in attrs.iter() {
                match &attr.name.local[..] {
                    "class" => {
                        a_cls.extend(attr.value.split_whitespace());
                    },
                    "id" => {
                        a_id.push(&attr.value[..]);
                    },
                    _ => {
                        a_props.push(format!("{}=\"{}\"", attr.name.local, attr.value));
                    }
                }
            }
            if a_tag == "div" && (a_id.len() > 0 || a_cls.len() > 0) {
                a_tag = "";
            }
            print!("{:indent$}{}{}{}{}", "", a_tag, a_id.join("#"), a_cls.join("."), a_props.join(" "), indent=indent);
            if node.children.borrow().iter().len() > 0 {
                println!(" {{");
                for child in node.children.borrow().iter() {
                    walk(indent + 4, child);
                }
                println!("{:indent$}}}", "", indent=indent);
            } else { println!(";") }
        },

        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

}

// FIXME: Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

pub fn is_empty(s: &str) -> bool {
    s.chars().fold(true, |a,b| a && b.is_whitespace())
}

fn main() {
    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();
    walk(0, &dom.document);

    if !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.iter() {
            println!("    {}", err);
        }
    }
}