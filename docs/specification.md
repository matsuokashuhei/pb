# CLI Command `pb` Specification

## Overview
A CLI command that displays a progress bar. Visualizes progress in real-time by specifying start time and end time.

## Command Name
`pb`

## Basic Syntax
```
pb --start <start_time> --end <end_time> [--interval <seconds>] [--help]
```

## Options

### Required Options
- `--start`, `-s`: Start time (required)
- `--end`, `-e`: End time (required)

### Optional
- `--interval`, `-i`: Update interval (seconds), default: 60 seconds
- `--help`, `-h`: Display help message

## Time Formats

Supports the following 3 formats:

1. **Date**: `2025-07-21`
2. **Date and time**: `2025-07-21 00:00:00`
3. **Relative time**:
   - `30m` (30 minutes)
   - `2h` (2 hours)
   - `1d` (1 day)

## Progress Bar Display

### Display Format
```
[████████████████████████████████████████] 100%
```

### Specifications
- **Width**: Fixed 40 characters
- **Update interval**: User configurable (default 60 seconds)
- **When exceeding 100%**: Display in red color
- **Termination condition**: Continue displaying even after reaching 100% (stop with Ctrl+C)

## Usage Examples

### Basic Usage
```bash
# Date and time specification
pb --start "2025-07-21 10:00:00" --end "2025-07-21 18:00:00"

# Date specification (time treated as 00:00:00)
pb --start "2025-07-21" --end "2025-07-22"

# Relative time specification
pb --start "2025-07-21 10:00:00" --end "8h"

# Update interval specification
pb --start "2025-07-21 10:00:00" --end "2025-07-21 18:00:00" --interval 30
```

### Help Display
```bash
pb --help
pb -h
```

## Error Handling

Display error messages and exit in the following cases:

1. **Start time > End time**
   - Error message: "Start time is later than end time"

2. **Invalid time format**
   - Error message: "Invalid time format: [input value]"

3. **End time already passed**
   - Error message: "The specified end time has already passed"

4. **Invalid relative time format**
   - Error message: "Invalid relative time format: [input value]"

5. **Missing required options**
   - Error message: "--start and --end options are required"

## Technical Specifications

### Operating Environment
- macOS, Linux, Windows
- Terminal environment

### Progress Bar Calculation
```
Progress rate = (Current time - Start time) / (End time - Start time) × 100
```

### Color Specification
- **Normal**: Default color
- **When exceeding 100%**: Red color (ANSI escape code: `\033[31m`)

## Implementation Notes

### Relative Time Parsing
- Regular expression: `^(\d+)([mhd])$`
- Conversion:
  - `m`: Minutes → seconds conversion (×60)
  - `h`: Hours → seconds conversion (×3600)
  - `d`: Days → seconds conversion (×86400)

### Real-time Updates
- Redraw progress bar at specified intervals
- Cursor position control to prevent flickering
- SIGINT (Ctrl+C) termination handling
