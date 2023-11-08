use std::collections::HashMap;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Delta {
    ops: Vec<DeltaOperation>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Delta {
    pub fn new(ops: Vec<DeltaOperation>) -> Delta {
        Delta { ops }
    }
}

impl Delta {
    pub fn ops(&self) -> &[DeltaOperation] {
        &self.ops
    }

    pub fn insert(
        &mut self,
        text_or_embed: DeltaOperationInsert,
        attributes: Option<HashMap<String, FlexMapValue>>,
    ) -> &mut Self {
        self
    }

    pub fn retain(
        &mut self,
        retain: u32,
        attributes: Option<HashMap<String, FlexMapValue>>,
    ) -> &mut Self {
        if retain <= 0 {
            return self;
        }

        self
    }

    pub fn delete(&mut self, delete: u32) -> &mut Self {
        if delete <= 0 {
            return self;
        }

        self
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum FlexMapValue {
    String(String),
    Bool(bool),
    Int(u32),
    Float(f32),
    Object(HashMap<String, FlexMapValue>),
    None,
}

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum DeltaOperationType {
    Insert,
    Delete,
    Retain,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct DeltaOperationInsert {
    pub string: Option<String>,
    pub embed: Option<HashMap<String, FlexMapValue>>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl DeltaOperationInsert {
    pub fn new_string(s: String) -> Self {
        Self {
            string: Some(s),
            embed: None,
        }
    }

    pub fn new_embed(e: HashMap<String, FlexMapValue>) -> Self {
        Self {
            string: None,
            embed: Some(e),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DeltaOperation {
    pub insert: Option<DeltaOperationInsert>,
    pub delete: Option<u32>,
    pub retain: Option<u32>,
    pub attributes: Option<HashMap<String, FlexMapValue>>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl DeltaOperation {
    pub fn insert_string(
        insert: &str,
        attributes: Option<HashMap<String, FlexMapValue>>,
    ) -> DeltaOperation {
        DeltaOperation::new(
            Some(DeltaOperationInsert::new_string(insert.into())),
            None,
            None,
            attributes.into(),
        )
    }

    pub fn insert_embed(
        insert: HashMap<String, FlexMapValue>,
        attributes: Option<HashMap<String, FlexMapValue>>,
    ) -> DeltaOperation {
        DeltaOperation::new(
            Some(DeltaOperationInsert::new_embed(insert)),
            None,
            None,
            attributes.into(),
        )
    }

    pub fn retain(
        retain: u32,
        attributes: Option<HashMap<String, FlexMapValue>>,
    ) -> DeltaOperation {
        DeltaOperation::new(None, None, retain.into(), attributes.into())
    }

    pub fn delete(delete: u32) -> DeltaOperation {
        DeltaOperation::new(None, delete.into(), None, None)
    }

    fn new(
        insert: Option<DeltaOperationInsert>,
        delete: Option<u32>,
        retain: Option<u32>,
        attributes: Option<HashMap<String, FlexMapValue>>,
    ) -> DeltaOperation {
        let validated_attributes = match attributes {
            Some(delta_attribute) if delta_attribute.len() > 0 => {
                Some(delta_attribute)
            }
            _ => None,
        };

        DeltaOperation {
            insert,
            delete,
            retain,
            attributes: validated_attributes,
        }
    }
}
