use quick_xml::events::Event;

use crate::deserializer::GetOrErr;
use crate::{
    deserializer::{Parseable, PrototypingToString},
    error::Error,
    reader::PeekableReader,
};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub(crate) struct BlockVariable {
    pub kind: VariableKind,
    pub formal_parameter: String,
    pub negated: bool,
    pub ref_local_id: Option<usize>,
    pub edge: Option<Edge>,
    pub storage: Option<Storage>,
    pub enable: Option<bool>,
}

#[derive(Debug)]
pub(crate) enum Edge {
    Falling,
    Rising,
}

#[derive(Debug)]
pub(crate) enum Storage {
    Set,
    Reset,
}

impl BlockVariable {
    pub fn new(hm: HashMap<String, String>, kind: VariableKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            formal_parameter: hm.get_or_err("formalParameter")?,
            negated: hm.get_or_err("negated").map(|it| it == "true")?,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
            edge: hm.get("edge").map(|it| it.parse()).transpose()?,
            storage: hm.get("storage").map(|it| it.parse()).transpose()?,
            enable: hm.get("enable").map(|it| it == "true"),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum VariableKind {
    Input,
    Output,
    InOut,
}

#[derive(Debug)]
pub(crate) struct FunctionBlockVariable {
    pub kind: VariableKind,
    pub local_id: usize,
    pub negated: bool,
    pub expression: String,
    pub execution_order_id: Option<usize>,
    pub ref_local_id: Option<usize>,
}

impl FunctionBlockVariable {
    pub fn new(hm: HashMap<String, String>, kind: VariableKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            negated: hm.get_or_err("negated").map(|it| it == "true")?,
            expression: hm.get_or_err("expression")?,
            execution_order_id: hm.get("executionOrderId").map(|it| it.parse()).transpose()?,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
        })
    }
}

impl TryFrom<&[u8]> for VariableKind {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"inputVariables" | b"inVariable" => Ok(VariableKind::Input),
            b"outputVariables" | b"outVariable" => Ok(VariableKind::Output),
            b"inOutVariables" | b"inOutVariable" => Ok(VariableKind::InOut),
            _ => {
                let value = std::str::from_utf8(value).map_err(Error::Encoding)?;
                Err(Error::UnexpectedElement(value.to_string()))
            }
        }
    }
}

// TODO: these impls should probably return a parse error instead of UnexpectedElement?

impl FromStr for Edge {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "falling" => Ok(Edge::Falling),
            "rising" => Ok(Edge::Rising),
            _ => Err(Error::UnexpectedElement(s.to_string())),
        }
    }
}

impl FromStr for Storage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "set" => Ok(Storage::Set),
            "reset" => Ok(Storage::Reset),
            _ => Err(Error::UnexpectedElement(s.to_string())),
        }
    }
}

impl Parseable for FunctionBlockVariable {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        // peek next token to determine variable kind
        // token will be consumed when extracting attributes later
        let next = reader.peek()?;
        let kind = match &next {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"inVariable" => VariableKind::Input,
                b"outVariable" => VariableKind::Output,
                b"inOutVariable" => VariableKind::InOut,
                _ => unreachable!(),
            },

            _ => unreachable!(),
        };

        let mut attributes = reader.attributes()?;
        loop {
            match reader.peek()? {
                Event::Text(tag) => {
                    attributes.insert("expression".into(), tag.as_ref().try_to_string()?);
                    reader.consume()?;
                }

                Event::End(tag) => match tag.name().as_ref() {
                    b"inVariable" | b"outVariable" => {
                        reader.consume()?;
                        break;
                    }
                    _ => reader.consume()?,
                },

                _ => reader.consume()?,
            }
        }

        FunctionBlockVariable::new(attributes, kind)
    }
}

impl Parseable for BlockVariable {
    type Item = Vec<Self>;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let kind = match reader.next()? {
            Event::Start(tag) | Event::Empty(tag) => VariableKind::try_from(tag.name().as_ref())?,
            _ => unreachable!(),
        };

        let mut res = vec![];

        loop {
            match reader.peek()? {
                Event::Start(tag) if tag.name().as_ref() == b"variable" => {
                    let attributes = visit_variable(reader)?;
                    res.push(BlockVariable::new(attributes, kind)?);
                }

                Event::End(tag)
                    if matches!(
                        tag.name().as_ref(),
                        b"inputVariables" | b"outputVariables" | b"inOutVariables"
                    ) =>
                {
                    reader.consume()?;
                    return Ok(res);
                }

                Event::Eof => {
                    return Err(Error::UnexpectedEndOfFile(vec![
                        b"inputVariables",
                        b"outputVariables",
                        b"inOutVariables",
                    ]))
                }
                _ => reader.consume()?,
            };
        }
    }
}

fn visit_variable(reader: &mut PeekableReader) -> Result<HashMap<String, String>, Error> {
    let mut attributes = HashMap::new();
    loop {
        match reader.peek()? {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"variable" | b"connection" => attributes.extend(reader.attributes()?),
                _ => reader.consume()?,
            },

            Event::End(tag) if tag.name().as_ref() == b"variable" => {
                reader.consume()?;
                break;
            }
            _ => reader.consume()?,
        }
    }

    Ok(attributes)
}
