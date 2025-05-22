# kick
**kick** simple and minimal project template CLI tool to create quick projects
It uses minijinja library to place arguments with placeholders
it doesnt provide any default templates

> this is my first rust project in attempt to learn rust

## Usage
`kick new <template_name> <project_name> [--arg value]...`
`kick list`
`kick help`

## How create custom Template
Templates are stored in config dir as `~/.config/kick/<template_name>`

create directry named c in config dir and create main.c and config.c
### config.toml example
```toml
name = "c" # language name
description = "simple c project"

[variables]
author = "AUTHOR_NAME"
```
### main.c
```c
#include <stdio.h>

int main(void){
  printf("Project {{projectName}} created by {{author}}");
  return 0;
}

```
