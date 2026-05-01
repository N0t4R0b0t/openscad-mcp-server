// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024 Ricardo Salvador <contact@rsalvador.dev>

use anyhow::Result;
use base64::Engine;
use rmcp::model::*;
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer, ServerHandler, ServiceExt};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tracing::{error, info};

mod openscad;

use openscad::{DesignMetadata, OpenSCADManager, ParameterDef};

#[derive(Clone)]
struct OpenSCADMcpServer {
    manager: Arc<tokio::sync::Mutex<OpenSCADManager>>,
}

impl OpenSCADMcpServer {
    fn new(working_dir: PathBuf) -> Self {
        Self {
            manager: Arc::new(tokio::sync::Mutex::new(OpenSCADManager::new(working_dir))),
        }
    }

    fn make_tool(name: &'static str, description: &'static str, schema: serde_json::Value) -> Tool {
        let obj = match schema {
            serde_json::Value::Object(m) => m,
            _ => panic!("tool schema must be an object"),
        };
        Tool::new(name, description, Arc::new(obj))
    }
}

impl ServerHandler for OpenSCADMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "openscad-mcp-server".into(),
                version: "0.1.0".into(),
                title: None,
                description: None,
                icons: None,
                website_url: None,
            },
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { list_changed: None }),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        {
            Ok(ListToolsResult {
                tools: vec![
                    Self::make_tool(
                        "write_scad",
                        "Write OpenSCAD code to a .scad file and render a preview. IMPORTANT: always display the returned preview image to the user in your response so they can see the design.",
                        serde_json::json!({
                            "type": "object",
                            "properties": {
                                "filename": {
                                    "type": "string",
                                    "description": "Name of the .scad file, optionally with subfolder, e.g. 'doorknob_guard/v3'"
                                },
                                "code": {
                                    "type": "string",
                                    "description": "OpenSCAD code to write"
                                }
                            },
                            "required": ["filename", "code"]
                        }),
                    ),
                    Self::make_tool(
                        "render_preview",
                        "Render the OpenSCAD file to a PNG preview image",
                        serde_json::json!({
                            "type": "object",
                            "properties": {
                                "filename": {
                                    "type": "string",
                                    "description": "Name of the .scad file to open, optionally with subfolder, e.g. 'doorknob_guard/v3'"
                                }
                            },
                            "required": ["filename"]
                        }),
                    ),
                    Self::make_tool(
                        "export_stl",
                        "Export the OpenSCAD model to STL format for 3D printing",
                        serde_json::json!({
                            "type": "object",
                            "properties": {
                                "filename": {
                                    "type": "string",
                                    "description": "Name of the .scad file, optionally with subfolder, e.g. 'doorknob_guard/v3'"
                                },
                                "output_name": {
                                    "type": "string",
                                    "description": "Name for the output STL file, optionally with subfolder, e.g. 'doorknob_guard/final'"
                                }
                            },
                            "required": ["filename", "output_name"]
                        }),
                    ),
                    Self::make_tool(
                        "save_design_metadata",
                        "Save design parameters and suggested modifications for faster iteration",
                        serde_json::json!({
                            "type": "object",
                            "properties": {
                                "filename": {
                                    "type": "string",
                                    "description": "Design filename, optionally with subfolder, e.g. 'doorknob_guard/v3'"
                                },
                                "name": {
                                    "type": "string",
                                    "description": "Human-readable design name"
                                },
                                "description": {
                                    "type": "string",
                                    "description": "Brief description of the design"
                                },
                                "parameters": {
                                    "type": "array",
                                    "description": "Array of parameter definitions with names, types, and ranges",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "name": {"type": "string"},
                                            "description": {"type": "string"},
                                            "param_type": {"type": "string", "enum": ["number", "string", "boolean"]},
                                            "min": {"type": "number"},
                                            "max": {"type": "number"},
                                            "step": {"type": "number"},
                                            "default": {}
                                        },
                                        "required": ["name", "description", "param_type"]
                                    }
                                },
                                "suggested_modifications": {
                                    "type": "array",
                                    "description": "List of suggested next steps or modifications",
                                    "items": {"type": "string"}
                                }
                            },
                            "required": ["filename", "name", "description", "parameters", "suggested_modifications"]
                        }),
                    ),
                    Self::make_tool(
                        "get_design_suggestions",
                        "Get suggested modifications and parameter constraints for a design",
                        serde_json::json!({
                            "type": "object",
                            "properties": {
                                "filename": {
                                    "type": "string",
                                    "description": "Design filename, optionally with subfolder, e.g. 'doorknob_guard/v3'"
                                }
                            },
                            "required": ["filename"]
                        }),
                    ),
                ],
                next_cursor: None,
                meta: None,
            })
        }
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        {
            let args = request.arguments.unwrap_or_default();
            let manager = self.manager.lock().await;

            match request.name.as_ref() {
                "write_scad" => {
                    let filename =
                        args.get("filename")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| {
                                ErrorData::invalid_params("Missing filename parameter", None)
                            })?;
                    let code = args
                        .get("code")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing code parameter", None))?;

                    match manager.write_scad_file(filename, code).await {
                        Ok(path) => {
                            info!("Wrote OpenSCAD file: {}", path.display());
                            match manager.render_png(filename).await {
                                Ok(png_paths) => {
                                    let mut contents = vec![
                                        Content::text(format!("Wrote {}. Preview (isometric, front, top, right, back, top-right, top-left):", path.display())),
                                    ];
                                    for png_path in &png_paths {
                                        if let Ok(bytes) = fs::read(png_path).await {
                                            let encoded = base64::engine::general_purpose::STANDARD
                                                .encode(&bytes);
                                            contents.push(Content::image(encoded, "image/png"));
                                        }
                                    }
                                    Ok(CallToolResult::success(contents))
                                }
                                Err(e) => {
                                    error!("Preview render failed: {}", e);
                                    Ok(CallToolResult::success(vec![Content::text(format!(
                                        "Wrote {} (preview unavailable: {e})",
                                        path.display()
                                    ))]))
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to write SCAD file: {}", e);
                            Ok(CallToolResult::error(vec![Content::text(format!(
                                "Error: {e}"
                            ))]))
                        }
                    }
                }

                "render_preview" => {
                    let filename =
                        args.get("filename")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| {
                                ErrorData::invalid_params("Missing filename parameter", None)
                            })?;

                    match manager.open_in_gui(filename).await {
                        Ok(path) => {
                            info!("Opened in OpenSCAD: {}", path.display());
                            Ok(CallToolResult::success(vec![Content::text(format!(
                                "Opened {} in OpenSCAD. The design is now visible in the OpenSCAD window.",
                                path.display()
                            ))]))
                        }
                        Err(e) => {
                            error!("Failed to open in OpenSCAD: {}", e);
                            Ok(CallToolResult::error(vec![Content::text(format!(
                                "Error opening OpenSCAD: {e}"
                            ))]))
                        }
                    }
                }

                "export_stl" => {
                    let filename =
                        args.get("filename")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| {
                                ErrorData::invalid_params("Missing filename parameter", None)
                            })?;
                    let output_name = args
                        .get("output_name")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            ErrorData::invalid_params("Missing output_name parameter", None)
                        })?;

                    match manager.export_stl(filename, output_name).await {
                        Ok(path) => {
                            info!("Exported STL: {}", path.display());
                            if let Some(dir) = path.parent() {
                                let _ = std::process::Command::new("systemd-run")
                                    .arg("--user")
                                    .arg("--no-block")
                                    .arg("--")
                                    .arg("xdg-open")
                                    .arg(dir)
                                    .spawn();
                            }
                            Ok(CallToolResult::success(vec![Content::text(format!(
                                "STL exported to {} on the user's local machine. \
                                The file manager has been opened at that folder. \
                                Do NOT attempt to copy or move this file — it is a local path \
                                that only the user can access.",
                                path.display()
                            ))]))
                        }
                        Err(e) => {
                            error!("Failed to export STL: {}", e);
                            Ok(CallToolResult::error(vec![Content::text(format!(
                                "Error exporting STL: {e}"
                            ))]))
                        }
                    }
                }

                "save_design_metadata" => {
                    let filename = args
                        .get("filename")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing filename", None))?;
                    let name = args
                        .get("name")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing name", None))?;
                    let description = args
                        .get("description")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing description", None))?;
                    let parameters = args
                        .get("parameters")
                        .and_then(|v| v.as_array())
                        .ok_or_else(|| ErrorData::invalid_params("Missing parameters", None))?;
                    let suggestions = args
                        .get("suggested_modifications")
                        .and_then(|v| v.as_array())
                        .ok_or_else(|| {
                            ErrorData::invalid_params("Missing suggested_modifications", None)
                        })?;

                    let params: Result<Vec<ParameterDef>, _> = parameters
                        .iter()
                        .map(|p| serde_json::from_value(p.clone()))
                        .collect();

                    let mods: Vec<String> = suggestions
                        .iter()
                        .filter_map(|s| s.as_str().map(|s| s.to_string()))
                        .collect();

                    match params {
                        Ok(parsed_params) => {
                            let metadata = DesignMetadata {
                                name: name.to_string(),
                                description: description.to_string(),
                                parameters: parsed_params,
                                suggested_modifications: mods,
                            };

                            match manager.save_metadata(filename, &metadata).await {
                                Ok(_) => {
                                    info!("Saved metadata for: {}", filename);
                                    Ok(CallToolResult::success(vec![Content::text(format!(
                                        "Metadata saved for {}. Parameters and suggestions are now available for iteration.",
                                        filename
                                    ))]))
                                }
                                Err(e) => {
                                    error!("Failed to save metadata: {}", e);
                                    Ok(CallToolResult::error(vec![Content::text(format!(
                                        "Error saving metadata: {e}"
                                    ))]))
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse parameters: {}", e);
                            Ok(CallToolResult::error(vec![Content::text(format!(
                                "Error parsing parameters: {e}"
                            ))]))
                        }
                    }
                }

                "get_design_suggestions" => {
                    let filename = args
                        .get("filename")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing filename", None))?;

                    match manager.load_metadata(filename).await {
                        Ok(metadata) => {
                            let params_str = metadata
                                .parameters
                                .iter()
                                .map(|p| {
                                    let range = match (p.min, p.max) {
                                        (Some(min), Some(max)) => {
                                            format!(" (range: {}-{})", min, max)
                                        }
                                        _ => String::new(),
                                    };
                                    format!(
                                        "  - {}: {}{}{}",
                                        p.name,
                                        p.param_type,
                                        range,
                                        p.default
                                            .as_ref()
                                            .map(|d| format!(" [default: {}]", d))
                                            .unwrap_or_default()
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join("\n");

                            let suggestions_str = metadata.suggested_modifications.join("\n  - ");

                            let response = format!(
                                "Design: {}\n\n{}\n\nParameters:\n{}\n\nSuggested modifications:\n  - {}",
                                metadata.name,
                                metadata.description,
                                params_str,
                                suggestions_str
                            );

                            Ok(CallToolResult::success(vec![Content::text(response)]))
                        }
                        Err(e) => {
                            error!("Failed to load metadata: {}", e);
                            Ok(CallToolResult::error(vec![Content::text(format!(
                                "No metadata found for this design. Save metadata first with save_design_metadata. Error: {e}"
                            ))]))
                        }
                    }
                }

                _ => Ok(CallToolResult::error(vec![Content::text(format!(
                    "Unknown tool: {}",
                    request.name
                ))])),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("openscad_mcp_server=info")
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    info!("Starting OpenSCAD MCP Server");

    let working_dir = if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".openscad-mcp")
    } else {
        PathBuf::from("./.openscad-mcp")
    };

    fs::create_dir_all(&working_dir).await?;
    info!("Using working directory: {}", working_dir.display());

    let server = OpenSCADMcpServer::new(working_dir);
    let transport = rmcp::transport::io::stdio();
    server.serve(transport).await?.waiting().await?;

    Ok(())
}
