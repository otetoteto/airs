# airs

Rule file deployment tool for AI agents

## Overview

airs is a command-line tool for managing and deploying rule files (instruction files) for AI agents such as GitHub Copilot.
It allows you to place pre-prepared rule files (`.md` files) in the `.github/copilot-instructions.md` location of your project.


## Usage

```bash
# When specifying a directory for stored rule files
airs --store /path/to/rule/files
```

When you specify a rule file directory, you can select from the `.md` files in that directory.
The selected file will be copied to `.github/copilot-instructions.md` in the project root.

## License

This project is released under the MIT License.