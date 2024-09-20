<!-- This doc is to describe the cst command line tool -->
# CST Command Line Tool
This is a command line tool that can be used to interpret or compile (soon) C-Script

## Usage
Run the command line tool by running the following command:
```bash
./cst ./<path-to-file>.csf <flag(s)>
```
<!-- explain the flags -->
Flags:
- '-t': Display the tokenized version of the code
- '-a': Display the AST of the code
- '-c': Compile the code
- Interpreting the code is the default behavior

The file should have the extension `.rss`. You can find some example files in the `examples` directory.
