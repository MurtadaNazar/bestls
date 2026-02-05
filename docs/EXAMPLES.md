# bestls Usage Examples

## Basic Listing
```bash
# List current directory
bestls

# List specific directory
bestls -p /home/user/documents
```

## Sorting
```bash
# Sort by name (default)
bestls

# Sort by file size
bestls --sort size

# Sort by modification date
bestls --sort date
```

## Hidden Files
```bash
# Show all files including hidden (.)
bestls --all
bestls -a
```

## v1.3 – Tree View & Filtering

### Tree View
```bash
# Show recursive directory tree
bestls --tree

# Limit tree depth to 2 levels
bestls --tree --depth 2

# Tree with specific path
bestls -p src --tree --depth 3
```

### Extension Filtering
```bash
# Show only Rust source files
bestls --filter-ext rs

# Show multiple file types
bestls --filter-ext rs,md,toml

# Combine with other options
bestls -p src --filter-ext rs --sort size
bestls --filter-ext txt --all --compact
```

### Pattern Filtering
```bash
# Show files matching pattern
bestls --filter-name "*.rs"

# Show files starting with test
bestls --filter-name "test_*"

# Combined with extension
bestls --filter-name "*_config*"
```

### Size Filtering
```bash
# Files at least 1KB
bestls --min-size 1KB

# Files at most 10MB
bestls --max-size 10MB

# Files between 100 bytes and 1MB
bestls --min-size 100B --max-size 1MB

# Large files only
bestls --min-size 1MB

# Small files only
bestls --max-size 100KB
```

### Combined Filtering
```bash
# Rust files over 10KB
bestls --filter-ext rs --min-size 10KB

# All markdown files in current and subdirs
bestls --tree --filter-ext md

# Config files under 5KB
bestls --filter-name "*config*" --max-size 5KB

# Source files, sorted by size
bestls --filter-ext rs,ts,js --sort size
```

## v1.4 – Output Customization

### Compact Mode
```bash
# Single-column listing
bestls --compact

# Compact with filtering
bestls -p src --filter-ext rs --compact
```

### Column Customization
```bash
# Custom columns
bestls --columns name,size,date

# Only show names and types
bestls --columns name,type
```

### Output to File
```bash
# Export as JSON
bestls --format json --out results.json

# Pretty JSON export
bestls --format json-pretty --out data.json

# Compact mode to file
bestls --compact --out file_list.txt

# Filter results to file
bestls --filter-ext rs --json --out rust_files.json
```

### Format Selection
```bash
# Explicit table format (default)
bestls --format table

# Compact JSON
bestls --format json

# Pretty-printed JSON
bestls --format json-pretty

# Legacy flags still work
bestls --json
bestls --json-pretty
```

### Color Control
```bash
# Disable colors (for piping/logs)
bestls --no-color

# CI/CD friendly output
bestls --no-color --format json-pretty --out results.json

# No color with filtering
bestls -p src --filter-ext rs --no-color
```

## Advanced Combinations

### Find Large Rust Files
```bash
bestls -p src --filter-ext rs --min-size 5KB --sort size --no-color
```

### Export JSON of Markdown Files
```bash
bestls --filter-ext md --format json-pretty --out markdown_files.json
```

### List All Configuration Files
```bash
bestls --tree --filter-name "*config*" --max-size 50KB --compact
```

### Find New Files (sorted by date)
```bash
bestls --all --sort date --no-color --compact
```

### Export Test Files
```bash
bestls --tree --filter-name "*test*" --format json --out test_files.json
```

### Show Directory Tree with Limits
```bash
# Show structure 3 levels deep, larger files
bestls --tree --depth 3 --min-size 1KB
```

### Analyze Source Code
```bash
# All source files with details
bestls --tree --filter-ext rs,ts,js --sort size

# Large files only
bestls --tree --filter-ext rs --min-size 20KB
```

## Shell Completion

### Generate Completions
```bash
# Bash
bestls completion bash > ~/.local/share/bash-completion/completions/bestls

# Zsh
bestls completion zsh > ~/.zfunc/_bestls

# Fish
bestls completion fish > ~/.config/fish/completions/bestls.fish
```

### Use Completions
```bash
# After installation, use TAB to complete
bestls --f<TAB>              # Expands to --filter-*
bestls --sort <TAB>          # Shows sort options
bestls --format <TAB>        # Shows format options
```

## Common Workflows

### Audit Disk Usage
```bash
# Find all files over 1MB
bestls --tree --min-size 1MB --sort size --no-color
```

### Backup Management
```bash
# Export all files to JSON for archival
bestls --tree --format json-pretty --out backup_manifest.json
```

### Code Review Preparation
```bash
# List changed Rust files
bestls -p src --filter-ext rs --sort date

# Export for code analysis
bestls --tree --filter-ext rs --json --out source_files.json
```

### CI/CD Output
```bash
# Machine-readable format, no colors
bestls -p build --tree --format json --no-color > build_artifacts.json
```

### Quick Navigation
```bash
# Single-column for easy reading
bestls --compact

# Same but hidden files included
bestls --all --compact
```

## Tips & Tricks

1. **Pipe to grep**: Works great with `--no-color --compact`
   ```bash
   bestls --no-color --compact | grep test
   ```

2. **Combine filters**: All filters use AND logic
   ```bash
   bestls --filter-ext rs --min-size 5KB --max-size 50KB
   ```

3. **Use with watch**: Monitor directory changes
   ```bash
   watch -n 1 'bestls --tree --depth 2 --no-color'
   ```

4. **Export for processing**: JSON output is fully parseable
   ```bash
   bestls --format json | jq '.[] | select(.len_bytes > 1000000)'
   ```

5. **Create reports**: Combine options for comprehensive analysis
   ```bash
   bestls --tree --filter-ext rs --sort size --no-color > source_report.txt
   ```
