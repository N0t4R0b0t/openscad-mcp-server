# Deployment & Operations Guide

## Production Deployment

### System Requirements

- **OS**: macOS 10.15+, Ubuntu 18.04+, Windows 10+
- **CPU**: Any modern processor
- **RAM**: 512MB minimum, 2GB recommended
- **Disk**: 100MB for binary + 1GB for working directory
- **OpenSCAD**: Must be installed and in PATH

### Pre-deployment Checklist

- [ ] OpenSCAD is installed: `openscad --version`
- [ ] Rust toolchain is installed: `rustc --version`
- [ ] AGPL-3.0 license is accessible
- [ ] Claude Desktop is installed
- [ ] Backup your current `claude_desktop_config.json`

### Installation Steps

#### 1. Build from Source

```bash
git clone https://github.com/N0t4R0b0t/openscad-mcp-server.git
cd openscad-mcp-server
cargo build --release
```

Binary location: `target/release/openscad-mcp-server`

#### 2. Verify Binary Works

```bash
# Test that the binary runs (it will wait for stdin)
timeout 2 ./target/release/openscad-mcp-server || true
# Should exit cleanly without errors
```

#### 3. Configure Claude Desktop

**macOS/Linux Config**:
```bash
# Open or create config file
nano ~/.claude/claude_desktop_config.json
```

**Windows Config** (PowerShell):
```powershell
$config = "$env:APPDATA\Claude\claude_desktop_config.json"
notepad $config
```

**Config Content**:
```json
{
  "mcpServers": {
    "openscad": {
      "command": "/full/path/to/openscad-mcp-server"
    }
  }
}
```

**Important**: Use the **absolute path** from `pwd` or `cd` in your terminal.

#### 4. Restart Claude Desktop

Close completely and reopen Claude Desktop.

#### 5. Test the Connection

In Claude, ask:
```
"Hello! Can you create a simple test cube, 10mm on each side?"
```

Claude should respond that it's using the OpenSCAD tool.

### Post-deployment

#### Verify Installation

```bash
# Check the working directory exists
ls -la ~/.openscad-mcp/

# Check recent files
ls -ltr ~/.openscad-mcp/ | tail -5
```

#### Monitor Logs (if enabled)

```bash
# Set log level
export RUST_LOG=openscad_mcp_server=debug

# Run directly to see logs
/path/to/openscad-mcp-server
```

#### Update Procedure

```bash
cd openscad-mcp-server
git pull origin main
cargo build --release

# Restart Claude Desktop
# (or kill any running instances)
pkill openscad-mcp-server  # If running in foreground
```

## Troubleshooting

### Server Won't Start

**Symptom**: Claude shows "Server error" or "Tool unavailable"

**Solutions**:
1. Check config path is correct (use absolute path)
2. Verify binary is executable: `chmod +x /path/to/binary`
3. Check for typos in JSON config
4. Restart Claude Desktop completely
5. Check logs if running directly:
```bash
export RUST_LOG=openscad_mcp_server=info
/path/to/openscad-mcp-server 2>&1
```

### OpenSCAD Not Found

**Symptom**: "OpenSCAD not found in PATH"

**Solutions**:
```bash
# Verify installation
which openscad          # macOS/Linux
where openscad          # Windows PowerShell

# Add to PATH (if needed)
export PATH="/opt/openscad:$PATH"  # macOS (adjust path)

# Or install fresh
brew install openscad           # macOS
sudo apt-get install openscad   # Ubuntu
```

### Files Not Appearing in `~/.openscad-mcp/`

**Symptom**: Server says it wrote a file, but it's not there

**Solutions**:
1. Check directory exists: `ls -la ~/.openscad-mcp/`
2. Check permissions: `ls -la ~/` (look for `.openscad-mcp`)
3. Try creating directory manually: `mkdir -p ~/.openscad-mcp`
4. Check disk space: `df -h ~/`

### Rendering Takes Too Long

**Symptom**: Claude says "rendering" but it's stuck

**Solutions**:
1. Check if OpenSCAD process is running: `ps aux | grep openscad`
2. Kill stuck processes: `pkill openscad`
3. Try a simpler model first
4. Check for very complex geometry in the SCAD code

## Performance Optimization

### For Slow Machines

The default settings should work on most machines, but if you have a slow system:

1. **Smaller preview images**: OpenSCAD renders at full quality (slow on old GPUs)
2. **Simpler models**: Fewer polygons = faster rendering
3. **More RAM**: If you're swapping, add more memory

### For Fast Iteration

1. Keep OpenSCAD GUI open while designing (file watching)
2. Use descriptive variable names so Claude understands your intent
3. Ask Claude for incremental changes rather than complete rewrites

## Security Considerations

### Local Network

This server only listens on localhost (`127.0.0.1`). It is **not** accessible from other computers on your network.

### File Access

All files are in `~/.openscad-mcp/` (user's home directory). No other directories are accessed.

### Process Execution

Only OpenSCAD CLI is executed. No arbitrary commands.

### AGPL-3.0 Compliance

If you modify the server code and redistribute it, you must provide:
1. Source code availability
2. License text
3. List of modifications

Internal use doesn't require redistribution.

## Backup & Recovery

### Backing Up Designs

```bash
# Create a backup of all designs
mkdir -p ~/backups
tar czf ~/backups/openscad-mcp-$(date +%Y%m%d).tar.gz ~/.openscad-mcp/

# Or sync to cloud
cp -r ~/.openscad-mcp/ ~/Dropbox/openscad-backups/
```

### Restoring from Backup

```bash
# Restore from tar
tar xzf ~/backups/openscad-mcp-20240426.tar.gz -C ~/

# Or from cloud
cp -r ~/Dropbox/openscad-backups/.openscad-mcp ~/
```

### Cleaning Up Old Files

```bash
# Remove files older than 30 days
find ~/.openscad-mcp/ -type f -mtime +30 -delete

# Or manually
cd ~/.openscad-mcp/
rm old_model_*.scad old_model_*_preview.png
```

## Maintenance

### Regular Tasks

- **Weekly**: Check for new versions: `git fetch origin`
- **Monthly**: Review `~/.openscad-mcp/` disk usage
- **Quarterly**: Update dependencies: `cargo update && cargo build --release`

### Uninstalling

```bash
# Option 1: Just remove from Claude config
# Edit ~/.claude/claude_desktop_config.json and remove the openscad block

# Option 2: Full removal
rm -rf ~/.openscad-mcp/
rm /path/to/openscad-mcp-server
# Edit Claude config to remove the entry
```

## Getting Help

1. **Check existing issues**: https://github.com/N0t4R0b0t/openscad-mcp-server/issues
2. **Enable debug logging**: `export RUST_LOG=openscad_mcp_server=debug`
3. **Open a new issue** with:
   - Your OS and version
   - OpenSCAD version: `openscad --version`
   - Error message or logs
   - Steps to reproduce

## License

This software is distributed under AGPL-3.0. See LICENSE file for full text.
