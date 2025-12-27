TodoHunter

The purpose of this tool is simple. Scan a git repository's tracked files for single-line or block comments containing TODO items. The result is presented in a (probably) friendly way to the user.

How to use:
Tag comments in tracked source code with TODO. End the todo message with /td. 
Then just run from the cli: tdhunt

Arguments:
path (p): The path to the desired repository. Defaults to current directory.
tag (t): Custom tags added to TODO messages. For example, running tdhunt -t BM will then look for comments tagged with TODO_BM.

To build:
TODO: explain how to build this thing.
