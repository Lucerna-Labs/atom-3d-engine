#!/usr/bin/env python3
"""
Linux UAPI Header Parser

Extracts primitives from Linux kernel uapi headers:
- #define constants (grouped by prefix)
- Struct definitions
- Ioctl command encodings
- Include dependencies

Usage: python parse_uapi.py <header_path> [--output <output_dir>]
"""

import re
import sys
import os
import json
from pathlib import Path
from dataclasses import dataclass, field, asdict
from typing import List, Dict, Optional, Set, Tuple

@dataclass
class DefineConstant:
    name: str
    value: str
    comment: str = ""
    category: str = ""  # Auto-grouped by prefix

@dataclass
class StructField:
    type_name: str
    name: str
    array_size: Optional[str] = None
    comment: str = ""

@dataclass
class StructDefinition:
    name: str
    fields: List[StructField] = field(default_factory=list)
    comments: List[str] = field(default_factory=list)
    packed: bool = False

@dataclass
class IoctlCommand:
    macro: str
    direction: str  # NONE, READ, WRITE, INOUT
    type_char: str
    nr: int
    size: Optional[str] = None
    argtype: Optional[str] = None

@dataclass
class HeaderAnalysis:
    filename: str
    includes: List[str] = field(default_factory=list)
    defines: List[DefineConstant] = field(default_factory=list)
    structs: List[StructDefinition] = field(default_factory=list)
    ioctls: List[IoctlCommand] = field(default_factory=list)
    typedefs: List[str] = field(default_factory=list)
    enums: Dict[str, List[Tuple[str, str]]] = field(default_factory=dict)

# Regex patterns
DEFINE_PATTERN = re.compile(r'#define\s+(\w+)\s+(.+?)(?:\s*/\*\s*(.+?)\s*\*/)?$')
STRUCT_START_PATTERN = re.compile(r'(?:typedef\s+)?struct\s*(?:__attribute__\s*\(\s*\(\s*packed\s*\)\s*\))?\s*(\w*)\s*\{')
FIELD_PATTERN = re.compile(r'(\w+(?:\s*\*)?)\s+(\w+)(?:\s*\[\s*(.+?)\s*\])?\s*;')
IOCTL_PATTERN = re.compile(r'#define\s+(_IO[RW]?_BAD?|_IO)\s*\(\s*(\w+)\s*,\s*(\d+)\s*(?:,\s*(\w+))?\s*\)')
INCLUDE_PATTERN = re.compile(r'#include\s+[<"]([^>"]+)[>"]')
TYPEDEF_PATTERN = re.compile(r'typedef\s+(?:struct|enum|union)\s*\w*\s+(\w+)')
ENUM_START_PATTERN = re.compile(r'typedef\s+enum\s*(\w*)\s*\{')
ENUM_ENTRY_PATTERN = re.compile(r'(\w+)(?:\s*=\s*(.+?))?')

def parse_define(line: str) -> Optional[DefineConstant]:
    """Parse a #define directive."""
    match = DEFINE_PATTERN.match(line.strip())
    if match:
        name, value, comment = match.groups()
        return DefineConstant(
            name=name,
            value=value.strip().rstrip('\\'),
            comment=comment or ""
        )
    return None

def parse_struct_fields(content: str) -> Tuple[List[StructField], bool]:
    """Parse struct fields from content between { and }."""
    fields = []
    packed = False
    
    # Check for __attribute__((packed))
    if 'packed' in content:
        packed = True
    
    for line in content.split('\n'):
        line = line.strip()
        if not line or line.startswith('//'):
            continue
        
        # Remove inline comments
        if '/*' in line:
            line = line.split('/*')[0].strip()
        
        match = FIELD_PATTERN.match(line)
        if match:
            type_name, name, array_size = match.groups()
            fields.append(StructField(
                type_name=type_name.strip(),
                name=name,
                array_size=array_size
            ))
    
    return fields, packed

def extract_structs(content: str) -> List[StructDefinition]:
    """Extract struct definitions from content."""
    structs = []
    
    # Find struct blocks
    for match in STRUCT_START_PATTERN.finditer(content):
        struct_name = match.group(1)
        start = match.end()
        
        # Find matching closing brace
        brace_count = 1
        pos = start
        while pos < len(content) and brace_count > 0:
            if content[pos] == '{':
                brace_count += 1
            elif content[pos] == '}':
                brace_count -= 1
            pos += 1
        
        struct_content = content[start:pos-1]
        fields, packed = parse_struct_fields(struct_content)
        
        structs.append(StructDefinition(
            name=struct_name or f"anonymous_{len(structs)}",
            fields=fields,
            packed=packed
        ))
    
    return structs

def extract_ioctls(content: str) -> List[IoctlCommand]:
    """Extract ioctl command definitions."""
    ioctls = []
    
    for match in IOCTL_PATTERN.finditer(content):
        macro, type_char, nr, argtype = match.groups()
        
        # Determine direction
        if '_IOR' in macro:
            direction = 'READ'
        elif '_IOW' in macro:
            direction = 'WRITE'
        elif '_IOWR' in macro:
            direction = 'INOUT'
        else:
            direction = 'NONE'
        
        ioctls.append(IoctlCommand(
            macro=macro,
            direction=direction,
            type_char=type_char,
            nr=int(nr),
            argtype=argtype
        ))
    
    return ioctls

def parse_header(filepath: str) -> HeaderAnalysis:
    """Parse a single header file."""
    with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    analysis = HeaderAnalysis(filename=os.path.basename(filepath))
    
    # Extract includes
    for match in INCLUDE_PATTERN.finditer(content):
        analysis.includes.append(match.group(1))
    
    # Extract defines (grouped by prefix)
    for line in content.split('\n'):
        define = parse_define(line)
        if define:
            # Auto-categorize by prefix
            prefix_match = re.match(r'^([A-Z]+_[A-Z]+)_', define.name)
            if prefix_match:
                define.category = prefix_match.group(1)
            analysis.defines.append(define)
    
    # Extract structs
    analysis.structs = extract_structs(content)
    
    # Extract ioctls
    analysis.ioctls = extract_ioctls(content)
    
    # Extract typedefs
    for match in TYPEDEF_PATTERN.finditer(content):
        analysis.typedefs.append(match.group(1))
    
    # Extract enums
    for enum_match in ENUM_START_PATTERN.finditer(content):
        enum_name = enum_match.group(1) or f"anonymous_enum_{len(analysis.enums)}"
        entries = []
        
        # Find enum content
        start = enum_match.end()
        brace_count = 1
        pos = start
        while pos < len(content) and brace_count > 0:
            if content[pos] == '{':
                brace_count += 1
            elif content[pos] == '}':
                brace_count -= 1
            pos += 1
        
        enum_content = content[start:pos-1]
        for entry_match in ENUM_ENTRY_PATTERN.finditer(enum_content):
            entry_name = entry_match.group(1)
            entry_value = entry_match.group(2) or ""
            entries.append((entry_name, entry_value.strip()))
        
        analysis.enums[enum_name] = entries
    
    return analysis

def generate_report(analysis: HeaderAnalysis, output_dir: str):
    """Generate a markdown report for the parsed header."""
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)
    
    report = []
    report.append(f"# {analysis.filename}\n")
    report.append(f"**Source:** `{analysis.filename}`\n")
    
    # Includes
    if analysis.includes:
        report.append("\n## Includes\n")
        for inc in analysis.includes:
            report.append(f"- `{inc}`")
    
    # Defines summary
    if analysis.defines:
        report.append(f"\n## Defines ({len(analysis.defines)} total)\n")
        
        # Group by category
        categories: Dict[str, List[DefineConstant]] = {}
        for d in analysis.defines:
            cat = d.category or "UNCATEGORIZED"
            if cat not in categories:
                categories[cat] = []
            categories[cat].append(d)
        
        for cat, defines in sorted(categories.items()):
            report.append(f"\n### {cat} ({len(defines)})\n")
            report.append("| Name | Value | Comment |")
            report.append("|------|-------|---------|")
            for d in defines[:50]:  # Limit per category
                escaped_value = d.value.replace('|', '\\|').replace('\n', ' ')
                report.append(f"| `{d.name}` | `{escaped_value[:60]}` | {d.comment} |")
            if len(defines) > 50:
                report.append(f"\n*...and {len(defines) - 50} more*")
    
    # Structs
    if analysis.structs:
        report.append(f"\n## Structs ({len(analysis.structs)})\n")
        for s in analysis.structs:
            report.append(f"\n### `struct {s.name}`\n")
            if s.packed:
                report.append("*Packed structure*\n")
            report.append("| Type | Field | Array |")
            report.append("|------|-------|-------|")
            for f in s.fields:
                report.append(f"| `{f.type_name}` | `{f.name}` | `{f.array_size or '-'}` |")
    
    # Ioctls
    if analysis.ioctls:
        report.append(f"\n## Ioctl Commands ({len(analysis.ioctls)})\n")
        report.append("| Macro | Direction | Type | Nr | Argtype |")
        report.append("|-------|-----------|------|----|----|")
        for io in analysis.ioctls:
            report.append(f"| `{io.macro}` | {io.direction} | `{io.type_char}` | {io.nr} | `{io.argtype or '-'}` |")
    
    # Typedefs
    if analysis.typedefs:
        report.append(f"\n## Typedefs\n")
        for t in analysis.typedefs:
            report.append(f"- `{t}`")
    
    # Enums
    if analysis.enums:
        report.append(f"\n## Enums\n")
        for enum_name, entries in analysis.enums.items():
            report.append(f"\n### `{enum_name}`\n")
            for name, value in entries[:20]:
                val_str = f"= {value}" if value else ""
                report.append(f"- `{name}` {val_str}")
            if len(entries) > 20:
                report.append(f"\n*...and {len(entries) - 20} more*")
    
    # Write report
    report_file = output_path / f"{Path(analysis.filename).stem}_analysis.md"
    with open(report_file, 'w', encoding='utf-8') as f:
        f.write('\n'.join(report))
    
    print(f"Generated: {report_file}")
    
    # Also write JSON for programmatic access
    json_file = output_path / f"{Path(analysis.filename).stem}_data.json"
    with open(json_file, 'w', encoding='utf-8') as f:
        json.dump(asdict(analysis), f, indent=2)
    
    print(f"Generated: {json_file}")

def main():
    if len(sys.argv) < 2:
        print("Usage: python parse_uapi.py <header_path> [--output <output_dir>]")
        sys.exit(1)
    
    header_path = sys.argv[1]
    output_dir = sys.argv[3] if len(sys.argv) > 3 and sys.argv[2] == '--output' else './output'
    
    if not os.path.exists(header_path):
        print(f"Error: File not found: {header_path}")
        sys.exit(1)
    
    print(f"Parsing: {header_path}")
    analysis = parse_header(header_path)
    generate_report(analysis, output_dir)
    
    print(f"\nSummary:")
    print(f"  - Includes: {len(analysis.includes)}")
    print(f"  - Defines: {len(analysis.defines)}")
    print(f"  - Structs: {len(analysis.structs)}")
    print(f"  - Ioctls: {len(analysis.ioctls)}")
    print(f"  - Typedefs: {len(analysis.typedefs)}")
    print(f"  - Enums: {len(analysis.enums)}")

if __name__ == '__main__':
    main()
