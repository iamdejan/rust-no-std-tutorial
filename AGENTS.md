# AGENTS.md

## Code Generation

Make sure you adhere to the guide on this file to generate the code.

### Rules of engagement

1. You're allowed to create functions, structs and impl blocks to generate the code.
2. With the exception of import statements and Cargo macros (allow and deny), explain the flow of the program using comments. Make sure that you write not only what the program is doing, but why. It will help me to judge your work.
3. You have to add Rustdoc. The Rustdoc should contain what is the function for (basically the description), a brief summary of the steps, and input and output parameters. If your function has the ability to throw panic, please state it in the Rustdoc as well.
4. You're not allowed to add `#[allow(dead_code)]`.

## Code Validation

After generating the code, make sure you validate the code you generated.

### Steps to Validate

Run these commands in sequence:
1. `pixi run fmt`: format your code. This ensures that the code written by human and by you (coding agent) are consistent, following the style that Zig has provided.
2. `pixi run lint-ci`: ensure no linter errors.
3. `pixi run lint-fix`: if there is any linter error, fix it with this command.
4. `pixi run lint-ci`: recheck again, maybe there are linter errors that need manual fix.
5. `pixi run test`: build the code, then run all unit tests. This ensures that the code you generate pass all the unit tests.

## Update Documentation

After the validation is finished, update the project tree structure and file descriptions in README.md if needed. This is to ensure we always have updated documentation.
