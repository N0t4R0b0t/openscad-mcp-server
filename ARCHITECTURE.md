# Architecture

## Overview

OpenSCAD MCP Server is a lightweight Model Context Protocol (MCP) server that bridges Claude and OpenSCAD through a simple, local command-line interface.

```
┌─────────────────┐
│  Claude.ai      │
└────────┬────────┘
         │ (JSON-RPC via stdio)
         │
┌────────▼─────────────────────┐
│ OpenSCAD MCP Server           │
├───────────────────────────────┤
│ - write_scad tool             │
│ - render_preview tool         │
│ - export_stl tool             │
└────────┬────────┬─────────────┘
         │        │
    ┌────▼──┐  ┌──▼────────────┐
    │.scad  │  │ OpenSCAD CLI  │
    │files  │  │(render, export)
    └───────┘  └───────────────┘
```

## Components

### 1. **Main Server** (`src/main.rs`)

- Implements the MCP `ServerHandler` trait
- Manages tool registration and dispatch
- Handles stdio-based JSON-RPC communication
- Enforces file I/O through the `OpenSCADManager`

**Key responsibilities:**
- Parse tool requests from Claude
- Validate input parameters
- Delegate to manager for actual work
- Return results as JSON

### 2. **OpenSCAD Manager** (`src/openscad.rs`)

Encapsulates all interactions with OpenSCAD:

- **`write_scad_file(filename, code)`** - Writes code to a `.scad` file in the working directory
- **`render_preview(filename)`** - Invokes OpenSCAD CLI to generate PNG preview
- **`export_stl(filename, output_name)`** - Invokes OpenSCAD CLI to export STL

**Design pattern:** Manager owns the working directory and ensures all file operations are contained.

### 3. **Error Handling** (`src/error.rs`)

Custom error types for:
- Missing OpenSCAD installation
- File I/O errors
- Render failures

## Data Flow

### Writing a Model

```
Claude: "Create a cube 50mm on each side"
  ↓
MCP Server parses request
  ↓
call_tool("write_scad", { filename: "my_model", code: "cube([50, 50, 50]);" })
  ↓
OpenSCADManager::write_scad_file()
  ↓
Write to ~/.openscad-mcp/my_model.scad
  ↓
Return success + file path to Claude
```

### Rendering a Preview

```
Claude: "Show me a preview"
  ↓
MCP Server parses request
  ↓
call_tool("render_preview", { filename: "my_model" })
  ↓
OpenSCADManager::render_preview()
  ↓
Execute: openscad -o ~/.openscad-mcp/my_model_preview.png ~/.openscad-mcp/my_model.scad
  ↓
Return preview file path to Claude
```

### Exporting for Printing

```
Claude: "Export this as STL named final_part"
  ↓
MCP Server parses request
  ↓
call_tool("export_stl", { filename: "my_model", output_name: "final_part" })
  ↓
OpenSCADManager::export_stl()
  ↓
Execute: openscad -o ~/.openscad-mcp/final_part.stl ~/.openscad-mcp/my_model.scad
  ↓
Return STL file path to Claude
```

## File Organization

```
~/.openscad-mcp/
├── my_model.scad           # Source file
├── my_model_preview.png    # Generated preview
├── final_part.stl          # Exported model
└── ...
```

## Technology Stack

- **Language**: Rust (safe, fast, zero-cost abstractions)
- **MCP Framework**: Official `rmcp` SDK (v0.16)
- **Runtime**: Tokio (async/await)
- **Process Management**: tokio::process for OpenSCAD invocation
- **File I/O**: tokio::fs for async file operations
- **Logging**: tracing + tracing-subscriber
- **Error Handling**: anyhow + thiserror

## Design Decisions

### 1. **Local-only, no authentication**

Rationale: This runs on localhost and is controlled by the user's machine. Authentication adds no security benefit and increases complexity.

### 2. **Stdio transport**

Rationale: Simplest integration with Claude Desktop. Process supervision handled by Claude itself.

### 3. **Manager pattern**

Rationale: Centralizes file and process logic. Makes testing easier and prevents file operations from leaking into the MCP handler.

### 4. **Async/await**

Rationale: OpenSCAD rendering can take time. Async prevents blocking on I/O or subprocess execution.

### 5. **Single working directory**

Rationale: All files in one place simplifies cleanup and file discovery. User can easily browse results.

## Future Extensibility

The architecture supports adding new tools without changing the core:

1. Add a new method to `OpenSCADManager`
2. Register it in `list_tools()`
3. Handle it in `call_tool()` match statement

Example: adding a `diff_models` tool:

```rust
// In openscad.rs
impl OpenSCADManager {
    pub async fn diff_models(&self, model1: &str, model2: &str) -> Result<PathBuf> {
        // Implementation
    }
}

// In main.rs
"diff_models" => {
    let model1 = request.arguments.get("model1")...
    let model2 = request.arguments.get("model2")...
    match manager.diff_models(model1, model2).await {
        Ok(result) => { /* ... */ },
        Err(e) => { /* error handling */ },
    }
}
```

## Testing Strategy

### Unit Tests

Test `OpenSCADManager` methods in isolation:

```rust
#[tokio::test]
async fn test_write_scad_creates_file() {
    let manager = OpenSCADManager::new(temp_dir);
    manager.write_scad_file("test", "cube([10, 10, 10]);").await.unwrap();
    // Assert file exists
}
```

### Integration Tests

Test the full flow through the MCP server:

```rust
#[tokio::test]
async fn test_write_and_render() {
    // Create server, call write_scad, call render_preview, verify results
}
```

### Manual Testing

1. Start the server
2. Connect with Claude Desktop
3. Test each tool with various inputs

## Performance Considerations

- **File writes**: Async (non-blocking)
- **Subprocess execution**: Async (non-blocking)
- **Memory**: Minimal overhead (~10MB at rest)
- **OpenSCAD runtime**: Depends on model complexity (not controllable from server)

## Security

- **No network access**: Runs on localhost only
- **No authentication**: User controls access via process ownership
- **File isolation**: All files in a single user-owned directory
- **Process isolation**: Subprocess execution via tokio, standard security model
- **Input validation**: Basic parameter validation, relies on OpenSCAD for code safety

## Troubleshooting Guide

### "OpenSCAD not found"

**Problem**: OpenSCAD is not in PATH
**Solution**: Install OpenSCAD, ensure it's accessible: `which openscad` or `where openscad`

### Render fails silently

**Problem**: OpenSCAD CLI is invoked but no preview appears
**Solution**: 
1. Verify SCAD syntax (check error logs)
2. Ensure OpenSCAD version is recent
3. Try rendering the same file in OpenSCAD GUI

### File permissions

**Problem**: Can't read/write files in `~/.openscad-mcp/`
**Solution**: Check directory permissions: `ls -la ~/.openscad-mcp/`

## References

- [MCP Specification](https://modelcontextprotocol.io)
- [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [OpenSCAD Documentation](https://openscad.org/documentation.html)
- [Tokio Runtime](https://tokio.rs)
