# Quick Start Guide

Get up and running with OpenSCAD MCP Server in 5 minutes.

## Prerequisites

- Rust 1.70+ ([install here](https://rustup.rs/))
- OpenSCAD ([install here](https://openscad.org/downloads.html))

## Installation

### 1. Build

```bash
git clone https://github.com/N0t4R0b0t/openscad-mcp-server.git
cd openscad-mcp-server
cargo build --release
```

Your binary is now at: `target/release/openscad-mcp-server`

### 2. Configure Claude Desktop

**Find your config file:**

- **macOS/Linux**: `~/.claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

**Edit it to add:**

```json
{
  "mcpServers": {
    "openscad": {
      "command": "/absolute/path/to/target/release/openscad-mcp-server"
    }
  }
}
```

Use the full path from step 1. On macOS/Linux, use `pwd` in the project directory to get it:

```bash
pwd
# /Users/username/projects/openscad-mcp-server
# So use: /Users/username/projects/openscad-mcp-server/target/release/openscad-mcp-server
```

### 3. Restart Claude Desktop

Close it completely, then reopen. The server should now be available.

## First Model

In Claude, try:

```
Create a simple cube, 50mm on each side
```

Claude will:
1. Generate OpenSCAD code
2. Write it to `~/.openscad-mcp/model.scad`
3. Show you the file location

Now:
1. Open that `.scad` file in OpenSCAD
2. Watch the preview as you continue chatting
3. Ask Claude for changes in real-time

## Export for Printing

When you're happy with your design:

```
Export this as an STL file named "my_cube"
```

The file will be ready at `~/.openscad-mcp/my_cube.stl`

## Next Steps

- Read the full [README.md](README.md) for detailed documentation
- Check [OpenSCAD documentation](https://openscad.org/documentation.html) for advanced techniques
- See [CONTRIBUTING.md](CONTRIBUTING.md) to contribute improvements

## Troubleshooting

**Config file not found?**

Create it manually:

```bash
mkdir -p ~/.claude
cat > ~/.claude/claude_desktop_config.json << 'EOF'
{
  "mcpServers": {
    "openscad": {
      "command": "/path/to/openscad-mcp-server"
    }
  }
}
EOF
```

**OpenSCAD not found?**

Make sure it's installed and in your PATH:

```bash
which openscad  # macOS/Linux
where openscad  # Windows (PowerShell)
```

**Server won't start?**

Check logs in Claude Desktop (Settings → Help → Debug) or run directly to see errors:

```bash
./target/release/openscad-mcp-server
```

## Tips

- Keep `.scad` files open in OpenSCAD while designing
- Ask Claude for specific measurements: "cube that's exactly 50mm × 30mm × 20mm"
- Use variables for parametric design: "make it configurable with height=50, width=30"
- Export multiple iterations with different names to compare

Happy designing! 🎨🖨️
