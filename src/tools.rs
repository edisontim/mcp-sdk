use crate::types::{CallToolRequest, CallToolResponse, ToolDefinition, ToolResponseContent};
use anyhow::Result;
use std::{collections::HashMap, sync::Arc};

pub trait Tool: Send + Sync + 'static {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn input_schema(&self) -> serde_json::Value;
    fn call(&self, input: Option<serde_json::Value>) -> Result<CallToolResponse>;
    fn as_definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name(),
            description: Some(self.description()),
            input_schema: self.input_schema(),
        }
    }
}

#[derive(Default)]
pub struct Tools {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl Tools {
    pub fn add_tool(&mut self, tool: impl Tool) {
        self.tools.insert(tool.name(), Arc::new(tool));
    }

    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        self.tools
            .values()
            .map(|tool| tool.as_definition())
            .collect()
    }

    pub fn call_tool(&self, request: CallToolRequest) -> CallToolResponse {
        let tool = self.tools.get(&request.name);
        if tool.is_none() {
            return CallToolResponse {
                content: vec![ToolResponseContent::Text {
                    text: format!("Tool {} not found", request.name),
                }],
                is_error: Some(true),
                meta: None,
            };
        }
        let arguments = request.arguments;
        let result = tool.unwrap().call(arguments);
        if result.is_err() {
            return CallToolResponse {
                content: vec![ToolResponseContent::Text {
                    text: format!(
                        "Error calling tool {}: {}",
                        &request.name,
                        result.err().unwrap()
                    ),
                }],
                is_error: Some(true),
                meta: None,
            };
        }
        result.unwrap()
    }
}
