# Nottes CLI

A simple command-line note-taking application written in Rust. Nottes allows you
to create, edit, list, and read markdown notes with a local SQLite database for
indexing and organization.

## Features

- **Create notes**: Add new notes with titles and automatic markdown formatting
- **Edit notes**: Edit existing notes with fuzzy search selection (using fzf)
- **List notes**: Display all your notes
- **Local storage**: Notes are stored as markdown files in `~/.nottes/`
- **Database indexing**: SQLite database for fast note searching and organization
- **Editor integration**: Opens notes in your preferred editor (uses `$EDITOR` environment variable)
- **Tagging system**: Database schema supports note tagging (coming soon)

## Installation

### Prerequisites

- Rust 1.70+
- `fzf` (for fuzzy note selection when editing)

### Build from source

```bash
git clone https://github.com/iamcaye/nottes-cli.git
cd nottes-cli
cargo build --release
```

The binary will be available at `target/release/nottes-cli`.

### Install globally

```bash
cargo install --path .
```

## Usage

### Commands

#### Add a new note

```bash
nottes add "My Note Title"
```

Creates a new markdown file with the given title and opens it in your editor.

#### Edit an existing note

```bash
nottes edit "partial title"
```

Searches for notes matching the partial title. If multiple matches are found, uses fzf for selection.

#### List all notes

```bash
nottes list
```

Displays all your notes.

#### Read a specific note

```bash
nottes read <note-id>
```

Reads a note by its ID.

### Configuration

- **Storage location**: `~/.nottes/` (automatically created)
- **Database**: `~/.nottes/.db/nottes.db` (SQLite)
- **Editor**: Set your preferred editor with the `EDITOR` environment variable (defaults to `vi`)

### File Structure

```
~/.nottes/
├── .db/
│   └── nottes.db          # SQLite database for indexing
├── note_title_1.md        # Your markdown notes
├── note_title_2.md
└── ...
```

### Note Format

Notes are created with basic frontmatter:

```markdown
---
title: Your Note Title
---

# Your content here
```

## Development

### Project Structure

```
src/
├── main.rs        # CLI argument parsing and main logic
├── cli.rs         # Note operations (add, edit, list, read)
└── db.rs          # Database operations and schema
```

### Database Schema

- **notes**: Stores note metadata (id, title, slug, created_at)
- **tags**: Tag definitions (planned feature)
- **note_tags**: Many-to-many relationship between notes and tags
- **settings**: Application settings
- **notes_history**: Version history for notes (planned feature)

### Dependencies

- `clap`: Command-line argument parsing
- `rusqlite`: SQLite database operations
- `anyhow`: Error handling
- `shellexpand`: Path expansion for `~` in file paths

## TODO

### High Priority

- [x] Implement `list` command functionality
- [ ] Add proper error handling for missing notes
- [ ] Add tests for all CLI operations
- [ ] Improve fuzzy search with better matching algorithms

### Medium Priority

- [ ] Implement tagging system
  - [ ] Add tags to notes
  - [ ] Search notes by tags
  - [ ] Tag management commands
- [ ] Add search functionality
  - [ ] Search by title
  - [ ] Search by content
  - [ ] Search by tags
- [ ] Note management
  - [ ] Delete notes
  - [ ] Rename notes
  - [ ] Archive notes
- [ ] Export functionality
  - [ ] Export to different formats (HTML, PDF)
  - [ ] Backup all notes

### Low Priority

- [ ] Configuration file support
- [ ] Custom note templates
- [ ] Note history/versioning
- [ ] Integration with external editors (VS Code, etc.)
- [ ] Sync with cloud storage (optional)
- [ ] Advanced search with full-text search
- [ ] Note statistics and analytics
- [ ] Plugin system for extensibility

### Technical Improvements

- [ ] Better error messages
- [ ] Logging system
- [ ] Performance optimizations
- [ ] Cross-platform compatibility testing
- [ ] CI/CD pipeline
- [ ] Release automation
- [ ] Documentation improvements
- [ ] Code refactoring and cleanup

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is open source. Please add your preferred license.

## Author

Created by [iamcaye](https://github.com/iamcaye)
