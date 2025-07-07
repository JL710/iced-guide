#!/usr/bin/python3
"""
Github: https://github.com/JL710/mdbook-project-path/tree/main
"""
import sys
import json
from pathlib import Path

def generic_replace(source_path: str, content: str, keyword: str, path: Path) -> str:
    if len(content) < len(keyword) + 4:
        return content
    
    source_path: Path = Path(source_path)
    if source_path.is_absolute():
        raise RuntimeError(f"Paths must be relative, but {source_path} is not!")
    replacement = "./"
    if len(source_path.parents) > 1:
        replacement += "/".join([".." for _ in source_path.parents][1:]) + "/"
    replacement = Path(replacement) / path

    i = -1
    escape_count = 0
    while True:
        i+= 1
            
        if i > len(content) - len(keyword) - 4:
            break
        if (escape_count == 0 or escape_count % 2 == 0) and content[i:i+4+len(keyword)] == "{{" + keyword + "}}":
            content = content[:max(0, i)] + str(replacement.as_posix()) + content[i+4+len(keyword):]
        
        if content[i] == "\\":
            escape_count += 1
        else:
            escape_count = 0
    
    return content

def chapter_worker(chapter: dict, keyword: str, path: Path):
    chapter["Chapter"]["content"] = generic_replace(
        chapter["Chapter"]["source_path"], 
        chapter["Chapter"]["content"], 
        keyword, 
        path
    )
    
    for sub_item in chapter["Chapter"]["sub_items"]:
        chapter_worker(sub_item, keyword, path)

if __name__ == "__main__":
    if len(sys.argv) > 1:
        if sys.argv[1] == "supports":
            sys.exit()

    context, book = json.load(sys.stdin)
    
    for keyword, path in context["config"]["preprocessor"]["project-path"]["paths"]:
        for section in book["sections"]:
            chapter_worker(section, keyword, path)
    
    print(json.dumps(book))
