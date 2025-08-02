# pb User Guide

pb is a command-line progress bar tool that visualizes the progress between two points in time. It's perfect for tracking deadlines, monitoring time-based processes, or simply visualizing how much time has elapsed in a given period.

## Table of Contents

- [Quick Start](#quick-start)
- [Time Formats](#time-formats)
- [Usage Examples](#usage-examples)
- [Command Line Options](#command-line-options)
- [Use Cases](#use-cases)
- [Output Formats](#output-formats)
- [Performance Tips](#performance-tips)
- [Frequently Asked Questions](#frequently-asked-questions)

## Quick Start

The simplest way to use pb is to specify an end time:

```bash
pb --end "17:00:00"
```

This will show a progress bar from now until 5 PM today.

You can also use the traditional approach with both start and end times:

```bash
pb --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"
```

This will show a progress bar indicating how much time has elapsed between 9 AM and 5 PM today.

## Time Formats

pb supports multiple time formats for maximum flexibility:

### Date Format (YYYY-MM-DD)
```bash
pb --start "2025-01-27" --end "2025-01-28"
```
When only a date is provided, the time defaults to 00:00:00 (midnight).

### Datetime Format (YYYY-MM-DD HH:MM:SS)
```bash
pb --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"
```
Full datetime specification with hours, minutes, and seconds.

### Relative Time Format
Use relative time expressions for the end time:

- **Hours**: `1h`, `2h`, `24h`
- **Days**: `1d`, `7d`, `30d`
- **Minutes**: `30m`, `45m`, `120m`
- **Seconds**: `30s`, `60s`, `3600s`

```bash
# 8-hour workday starting now (simplified)
pb --end "8h"

# 8-hour workday with explicit start (traditional)
pb --start "2025-01-27 09:00:00" --end "8h"

# 7-day countdown from today
pb --end "2025-02-03"

# 7-day countdown with explicit start (traditional)
pb --start "2025-01-27" --end "7d"

# 30-minute meeting starting now
pb --end "30m"

# 30-minute meeting with explicit start (traditional)
pb --start "2025-01-27 14:00:00" --end "30m"
```

### Automatic Start Time Detection

When you omit the `--start` parameter, pb automatically determines the start time based on the end time format:

- **Time-containing formats** (e.g., "17:00:00", "2h", "+30m") → Start from current time
- **Date-only formats** (e.g., "2025-12-31") → Start from today at 00:00:00

This makes pb more intuitive for common scenarios like timers and countdowns.

## Usage Examples

### Work Day Progress
Track your work day progress (traditional method):
```bash
pb --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00" --interval 60
```

Track remaining work day from now:
```bash
pb --end "17:00:00" --interval 60
```

### Project Deadline
Monitor time remaining until a project deadline (traditional method):
```bash
pb --start "2025-01-20" --end "2025-02-15" --interval 3600
```

Monitor time remaining until deadline from today:
```bash
pb --end "2025-02-15" --interval 3600
```

### Meeting Timer
Track a meeting or presentation (traditional method):
```bash
pb --start "2025-01-27 14:00:00" --end "1h" --interval 30
```

1-hour meeting timer starting now:
```bash
pb --end "1h" --interval 30
```

### Study Session
Monitor a study or focus session (traditional method):
```bash
pb --start "2025-01-27 10:00:00" --end "2h" --interval 300
```

2-hour study session starting now:
```bash
pb --end "2h" --interval 300
```

### Event Countdown
Create a countdown to an important event (traditional method):
```bash
pb --start "2025-01-27" --end "2025-06-15" --interval 86400
```

Countdown to event from today:
```bash
pb --end "2025-06-15" --interval 86400
```

## Command Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--start` | `-s` | Start time (optional) | Auto-determined from end time |
| `--end` | `-e` | End time (required) | - |
| `--interval` | `-i` | Update interval in seconds | 60 |
| `--help` | `-h` | Show help message | - |
| `--version` | `-V` | Show version information | - |

### Update Intervals

The `--interval` option controls how frequently the progress bar updates:

- **1 second**: Very frequent updates, good for short durations
- **60 seconds** (default): Good balance for most use cases
- **300 seconds** (5 minutes): Suitable for longer periods
- **3600 seconds** (1 hour): Good for day-long or week-long tracking

```bash
# Update every second (for short-term monitoring, traditional)
pb --start "2025-01-27 14:00:00" --end "30m" --interval 1

# Update every second (30-minute timer from now)
pb --end "30m" --interval 1

# Update every 5 minutes (for longer sessions, traditional)
pb --start "2025-01-27 09:00:00" --end "8h" --interval 300

# Update every 5 minutes (8-hour session from now)
pb --end "8h" --interval 300

# Update every hour (for multi-day tracking)
pb --start "2025-01-27" --end "7d" --interval 3600
```

## Use Cases

### Time Management
- **Pomodoro Technique**: Track 25-minute focus sessions
- **Time Boxing**: Allocate specific time slots for tasks
- **Break Reminders**: Monitor work/break cycles

### Project Management
- **Milestone Tracking**: Visualize progress toward project milestones
- **Deadline Monitoring**: Keep track of approaching deadlines
- **Sprint Progress**: Monitor development sprint progress

### Personal Productivity
- **Study Sessions**: Track focused study time
- **Exercise Duration**: Monitor workout sessions
- **Habit Formation**: Track time-based habits

### Event Planning
- **Countdown Timers**: Create countdowns for events
- **Schedule Monitoring**: Track event durations
- **Time Awareness**: Maintain awareness of time passage

## Output Formats

### Terminal (TTY) Mode
When running in a terminal, pb displays a dynamic progress bar that updates in place:

```
pb - Progress Bar Tool
Start time: 2025-01-27 09:00:00
End time: 2025-01-27 17:00:00
Update interval: 60 seconds
Press Ctrl+C to exit

[████████░░░░░░░░░░░░░░░░░░░░] 32.5% (2h 36m elapsed, 5h 24m remaining)
```

### Non-TTY Mode
When output is redirected or piped, pb prints progress updates as separate lines:

```
[████████░░░░░░░░░░░░░░░░░░░░] 32.5% (2h 36m elapsed, 5h 24m remaining)
[█████████░░░░░░░░░░░░░░░░░░░] 35.0% (2h 48m elapsed, 5h 12m remaining)
[█████████░░░░░░░░░░░░░░░░░░░] 37.5% (3h 00m elapsed, 5h 00m remaining)
```

### Progress Bar Elements

The progress bar consists of:
- **Filled blocks** (█): Completed time
- **Empty blocks** (░): Remaining time
- **Percentage**: Exact completion percentage
- **Time information**: Elapsed and remaining time

### Color Coding
The progress bar uses colors to indicate status:
- **Green**: Normal progress (0-80%)
- **Yellow**: Nearing completion (80-100%)
- **Red**: Overtime (>100%)

## Performance Tips

### Choosing Update Intervals
- **Short durations** (< 1 hour): Use 1-30 second intervals
- **Medium durations** (1-8 hours): Use 1-5 minute intervals
- **Long durations** (> 8 hours): Use 15+ minute intervals

### Resource Usage
pb is designed to be lightweight:
- **CPU Usage**: Minimal, only during updates
- **Memory Usage**: Less than 10MB
- **Network**: No network usage required

### Terminal Performance
For best performance in terminals:
- Use modern terminal emulators that support ANSI colors
- Avoid very short intervals (< 1 second) for long-running sessions

## Frequently Asked Questions

### Q: Can I pause and resume the progress bar?
A: pb tracks real time, so it cannot be paused. If you need to pause tracking, stop pb (Ctrl+C) and restart it later with adjusted times.

### Q: What happens when the end time is reached?
A: pb will show 100% completion and exit automatically. If the current time exceeds the end time, it will show >100% in red.

### Q: Can I use pb for past time periods?
A: Yes, but the progress will immediately show 100% since the time period has already elapsed.

### Q: Does pb work across time zones?
A: pb uses the local system time. Ensure your system clock is set correctly for accurate progress tracking.

### Q: Can I customize the progress bar appearance?
A: Currently, pb uses a fixed format optimized for readability. Color output can be controlled by your terminal's color support.

### Q: How accurate is the time calculation?
A: pb uses high-precision time calculations and updates based on your specified interval. Accuracy depends on your system clock.

### Q: Can I run multiple pb instances?
A: Yes, you can run multiple pb instances in different terminals to track multiple time periods simultaneously.

### Q: What if I specify an invalid time format?
A: pb will display a clear error message indicating the expected format and exit with a non-zero status code.