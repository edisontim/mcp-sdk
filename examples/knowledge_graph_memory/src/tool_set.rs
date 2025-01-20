use crate::types::{
    AddObservationParams, DeleteObservationParams, Entity, KnowledgeGraph, Relation,
};
use anyhow::Result;
use mcp_sdk::{
    tools::{Tool, Tools},
    types::{CallToolResponse, ToolResponseContent},
};
use serde_json::json;
use std::sync::{Arc, Mutex};

pub fn tool_set(kg: Arc<Mutex<KnowledgeGraph>>, memory_file_path: String) -> Tools {
    let mut tools = Tools::default();
    tools.add_tool(CreateEntitiesTool::new(
        kg.clone(),
        memory_file_path.clone(),
    ));
    tools.add_tool(CreateRelationsTool::new(
        kg.clone(),
        memory_file_path.clone(),
    ));
    tools.add_tool(ReadGraphTool::new(kg.clone()));
    tools.add_tool(AddObservationsTool::new(
        kg.clone(),
        memory_file_path.clone(),
    ));
    tools.add_tool(DeleteEntitiesTool::new(
        kg.clone(),
        memory_file_path.clone(),
    ));
    tools.add_tool(DeleteObservationsTool::new(
        kg.clone(),
        memory_file_path.clone(),
    ));
    tools.add_tool(DeleteRelationsTool::new(
        kg.clone(),
        memory_file_path.clone(),
    ));
    tools.add_tool(SearchNodesTool::new(kg.clone()));
    tools.add_tool(OpenNodesTool::new(kg.clone()));
    tools
}

pub struct CreateEntitiesTool {
    kg: Arc<Mutex<KnowledgeGraph>>,
    memory_file_path: String,
}

impl CreateEntitiesTool {
    pub fn new(kg: Arc<Mutex<KnowledgeGraph>>, memory_file_path: String) -> Self {
        Self {
            kg,
            memory_file_path,
        }
    }
}

impl Tool for CreateEntitiesTool {
    fn name(&self) -> String {
        "create_entities".to_string()
    }

    fn description(&self) -> String {
        "Create multiple new entities".to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
           "type":"object",
           "properties":{
              "entities":{
                 "type":"array",
                 "items":{
                    "type":"object",
                    "properties":{
                       "name":{"type":"string"},
                       "entityType":{"type":"string"},
                       "observations":{
                          "type":"array", "items":{"type":"string"}
                       }
                    },
                    "required":["name","entityType","observations"]
                 }
              }
           },
           "required":["entities"]
        })
    }

    fn call(&self, input: Option<serde_json::Value>) -> Result<CallToolResponse> {
        let args = input.unwrap_or_default();
        let entities = args
            .get("entities")
            .ok_or(anyhow::anyhow!("missing arguments `entities`"))?;
        let entities: Vec<Entity> = serde_json::from_value(entities.clone())?;
        let created = self.kg.lock().unwrap().create_entities(entities)?;
        self.kg
            .lock()
            .unwrap()
            .save_to_file(&self.memory_file_path)?;
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!(created).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

// Similar pattern for other tools...
pub struct CreateRelationsTool {
    kg: Arc<Mutex<KnowledgeGraph>>,
    memory_file_path: String,
}

impl CreateRelationsTool {
    pub fn new(kg: Arc<Mutex<KnowledgeGraph>>, memory_file_path: String) -> Self {
        Self {
            kg,
            memory_file_path,
        }
    }
}

impl Tool for CreateRelationsTool {
    fn name(&self) -> String {
        "create_relations".to_string()
    }

    fn description(&self) -> String {
        "Create multiple new relations".to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
           "type":"object",
           "properties":{
              "relations":{
                 "type":"array",
                 "items":{
                    "type":"object",
                    "properties":{
                       "from":{"type":"string"},
                       "to":{"type":"string"},
                       "relationType":{"type":"string"}
                    },
                    "required":["from","to","relationType"]
                 }
              }
           },
           "required":["relations"]
        })
    }

    fn call(&self, input: Option<serde_json::Value>) -> Result<CallToolResponse> {
        let args = input.unwrap_or_default();
        let relations = args
            .get("relations")
            .ok_or(anyhow::anyhow!("missing arguments `relations`"))?;
        let relations: Vec<Relation> = serde_json::from_value(relations.clone())?;
        let created = self.kg.lock().unwrap().create_relations(relations)?;
        self.kg
            .lock()
            .unwrap()
            .save_to_file(&self.memory_file_path)?;
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!(created).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

// Continue implementing for each tool (AddObservationsTool, DeleteEntitiesTool, etc.)
// I'll show one more as example:

pub struct ReadGraphTool {
    kg: Arc<Mutex<KnowledgeGraph>>,
}

impl ReadGraphTool {
    pub fn new(kg: Arc<Mutex<KnowledgeGraph>>) -> Self {
        Self { kg }
    }
}

impl Tool for ReadGraphTool {
    fn name(&self) -> String {
        "read_graph".to_string()
    }

    fn description(&self) -> String {
        "Read the entire knowledge graph".to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {}
        })
    }

    fn call(&self, _input: Option<serde_json::Value>) -> Result<CallToolResponse> {
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!(*self.kg.lock().unwrap()).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

pub struct AddObservationsTool {
    kg: Arc<Mutex<KnowledgeGraph>>,
    memory_file_path: String,
}

impl AddObservationsTool {
    pub fn new(kg: Arc<Mutex<KnowledgeGraph>>, memory_file_path: String) -> Self {
        Self {
            kg,
            memory_file_path,
        }
    }
}

impl Tool for AddObservationsTool {
    fn name(&self) -> String {
        "add_observations".to_string()
    }

    fn description(&self) -> String {
        "Add new observations to existing entities".to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "observations": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "entityName": {"type": "string"},
                            "contents": {
                                "type": "array",
                                "items": {"type": "string"}
                            }
                        },
                        "required": ["entityName", "contents"]
                    }
                }
            },
            "required": ["observations"]
        })
    }

    fn call(&self, input: Option<serde_json::Value>) -> Result<CallToolResponse> {
        let args = input.unwrap_or_default();
        let observations = args
            .get("observations")
            .ok_or(anyhow::anyhow!("missing arguments `observations`"))?;
        let observations: Vec<AddObservationParams> = serde_json::from_value(observations.clone())?;
        let results = self.kg.lock().unwrap().add_observations(observations)?;
        self.kg
            .lock()
            .unwrap()
            .save_to_file(&self.memory_file_path)?;
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!(results).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

pub struct DeleteEntitiesTool {
    kg: Arc<Mutex<KnowledgeGraph>>,
    memory_file_path: String,
}

impl DeleteEntitiesTool {
    pub fn new(kg: Arc<Mutex<KnowledgeGraph>>, memory_file_path: String) -> Self {
        Self {
            kg,
            memory_file_path,
        }
    }
}

impl Tool for DeleteEntitiesTool {
    fn name(&self) -> String {
        "delete_entities".to_string()
    }

    fn description(&self) -> String {
        "Delete multiple entities and their relations".to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "entityNames": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "required": ["entityNames"]
        })
    }

    fn call(&self, input: Option<serde_json::Value>) -> Result<CallToolResponse> {
        let args = input.unwrap_or_default();
        let entity_names = args
            .get("entityNames")
            .ok_or(anyhow::anyhow!("missing arguments `entityNames`"))?;
        let entity_names: Vec<String> = serde_json::from_value(entity_names.clone())?;
        let mut kg_guard = self.kg.lock().unwrap();
        kg_guard.delete_entities(entity_names)?;
        kg_guard.save_to_file(&self.memory_file_path)?;
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: "Entities deleted successfully".to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

pub struct DeleteObservationsTool {
    kg: Arc<Mutex<KnowledgeGraph>>,
    memory_file_path: String,
}

impl DeleteObservationsTool {
    pub fn new(kg: Arc<Mutex<KnowledgeGraph>>, memory_file_path: String) -> Self {
        Self {
            kg,
            memory_file_path,
        }
    }
}

impl Tool for DeleteObservationsTool {
    fn name(&self) -> String {
        "delete_observations".to_string()
    }

    fn description(&self) -> String {
        "Delete specific observations from entities".to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "deletions": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "entityName": {"type": "string"},
                            "observations": {
                                "type": "array",
                                "items": {"type": "string"}
                            }
                        },
                        "required": ["entityName", "observations"]
                    }
                }
            },
            "required": ["deletions"]
        })
    }

    fn call(&self, input: Option<serde_json::Value>) -> Result<CallToolResponse> {
        let args = input.unwrap_or_default();
        let deletions = args
            .get("deletions")
            .ok_or(anyhow::anyhow!("missing arguments `deletions`"))?;
        let deletions: Vec<DeleteObservationParams> = serde_json::from_value(deletions.clone())?;
        let mut kg_guard = self.kg.lock().unwrap();
        kg_guard.delete_observations(deletions)?;
        kg_guard.save_to_file(&self.memory_file_path)?;
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: "Observations deleted successfully".to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

pub struct DeleteRelationsTool {
    kg: Arc<Mutex<KnowledgeGraph>>,
    memory_file_path: String,
}

impl DeleteRelationsTool {
    pub fn new(kg: Arc<Mutex<KnowledgeGraph>>, memory_file_path: String) -> Self {
        Self {
            kg,
            memory_file_path,
        }
    }
}

impl Tool for DeleteRelationsTool {
    fn name(&self) -> String {
        "delete_relations".to_string()
    }

    fn description(&self) -> String {
        "Delete multiple relations from the graph".to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "relations": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "from": {"type": "string"},
                            "to": {"type": "string"},
                            "relationType": {"type": "string"}
                        },
                        "required": ["from", "to", "relationType"]
                    }
                }
            },
            "required": ["relations"]
        })
    }

    fn call(&self, input: Option<serde_json::Value>) -> Result<CallToolResponse> {
        let args = input.unwrap_or_default();
        let relations = args
            .get("relations")
            .ok_or(anyhow::anyhow!("missing arguments `relations`"))?;
        let relations: Vec<Relation> = serde_json::from_value(relations.clone())?;
        let mut kg_guard = self.kg.lock().unwrap();
        kg_guard.delete_relations(relations)?;
        kg_guard.save_to_file(&self.memory_file_path)?;
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: "Relations deleted successfully".to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

pub struct SearchNodesTool {
    kg: Arc<Mutex<KnowledgeGraph>>,
}

impl SearchNodesTool {
    pub fn new(kg: Arc<Mutex<KnowledgeGraph>>) -> Self {
        Self { kg }
    }
}

impl Tool for SearchNodesTool {
    fn name(&self) -> String {
        "search_nodes".to_string()
    }

    fn description(&self) -> String {
        "Search for nodes in the knowledge graph".to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "query": {"type": "string"}
            },
            "required": ["query"]
        })
    }

    fn call(&self, input: Option<serde_json::Value>) -> Result<CallToolResponse> {
        let args = input.unwrap_or_default();
        let query = args
            .get("query")
            .ok_or(anyhow::anyhow!("missing argument `query`"))?
            .as_str()
            .ok_or(anyhow::anyhow!("query must be a string"))?;
        let results = self.kg.lock().unwrap().search_nodes(query)?;
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!(results).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}

pub struct OpenNodesTool {
    kg: Arc<Mutex<KnowledgeGraph>>,
}

impl OpenNodesTool {
    pub fn new(kg: Arc<Mutex<KnowledgeGraph>>) -> Self {
        Self { kg }
    }
}

impl Tool for OpenNodesTool {
    fn name(&self) -> String {
        "open_nodes".to_string()
    }

    fn description(&self) -> String {
        "Open specific nodes by their names".to_string()
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "names": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "required": ["names"]
        })
    }

    fn call(&self, input: Option<serde_json::Value>) -> Result<CallToolResponse> {
        let args = input.unwrap_or_default();
        let names = args
            .get("names")
            .ok_or(anyhow::anyhow!("missing arguments `names`"))?;
        let names: Vec<String> = serde_json::from_value(names.clone())?;
        let results = self.kg.lock().unwrap().open_nodes(names)?;
        Ok(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: json!(results).to_string(),
            }],
            is_error: None,
            meta: None,
        })
    }
}
