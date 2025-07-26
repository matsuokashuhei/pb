# pb Usage Examples

This directory contains practical examples of how to use pb (progress bar tool) in various scenarios.

## Basic Examples

### Example 1: Work Day Progress
Track your 8-hour work day:

```bash
pb --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"
```

**Output**:
```
pb - Progress Bar Tool
Start time: 2025-01-27 09:00:00
End time: 2025-01-27 17:00:00
Update interval: 60 seconds
Press Ctrl+C to exit

[████████░░░░░░░░░░░░░░░░░░░░] 32.5% (2h 36m elapsed, 5h 24m remaining)
```

### Example 2: Meeting Timer
Monitor a 1-hour meeting:

```bash
pb --start "2025-01-27 14:00:00" --end "1h" --interval 30
```

### Example 3: Project Deadline
Track time until project deadline:

```bash
pb --start "2025-01-20" --end "2025-02-15" --interval 3600
```

## Advanced Examples

### Example 4: Study Session with Breaks
Create a 2-hour study session with 5-minute update intervals:

```bash
# Study session
pb --start "2025-01-27 10:00:00" --end "2h" --interval 300

# During break, track break time
pb --start "2025-01-27 12:00:00" --end "15m" --interval 60
```

### Example 5: Event Countdown
Multi-day countdown to an important event:

```bash
pb --start "2025-01-27" --end "2025-06-15" --interval 86400
```

### Example 6: Cooking Timer
Track cooking time with frequent updates:

```bash
pb --start "2025-01-27 18:30:00" --end "45m" --interval 1
```

## Scripting Examples

### Example 7: Automated Progress Monitoring
Use pb in a script to monitor progress:

```bash
#!/bin/bash
# monitor_workday.sh

echo "Starting work day monitoring..."
pb --start "$(date '+%Y-%m-%d 09:00:00')" --end "8h" --interval 300 | while read -r line; do
    echo "$(date): $line" >> work_progress.log
    
    # Send notification at 50% completion
    if echo "$line" | grep -q "50.0%"; then
        echo "Work day is 50% complete!" | wall
    fi
done
```

### Example 8: Background Progress Tracking
Run pb in the background and log progress:

```bash
#!/bin/bash
# background_timer.sh

# Start pb in background
pb --start "2025-01-27 14:00:00" --end "2h" --interval 60 > progress.log 2>&1 &
PB_PID=$!

echo "Progress bar started with PID: $PB_PID"
echo "Log file: progress.log"

# Monitor in real-time
tail -f progress.log &
TAIL_PID=$!

# Wait for pb to complete
wait $PB_PID

# Stop tailing
kill $TAIL_PID

echo "Progress monitoring completed!"
```

### Example 9: Multiple Timers
Track multiple time periods simultaneously:

```bash
#!/bin/bash
# multiple_timers.sh

# Start multiple pb instances
echo "Starting work timer..."
pb --start "2025-01-27 09:00:00" --end "8h" --interval 300 > work.log &

echo "Starting lunch break timer..."
pb --start "2025-01-27 12:00:00" --end "1h" --interval 60 > lunch.log &

echo "Starting meeting timer..."
pb --start "2025-01-27 14:00:00" --end "2h" --interval 120 > meeting.log &

# Monitor all logs
tail -f work.log lunch.log meeting.log
```

## Integration Examples

### Example 10: With Cron Jobs
Schedule automated progress tracking:

```bash
# Add to crontab (crontab -e)
# Start work day timer at 9 AM every weekday
0 9 * * 1-5 /usr/local/bin/pb --start "$(date '+%Y-%m-%d 09:00:00')" --end "8h" > /tmp/work_progress.log 2>&1 &

# Daily deadline reminder at 8 AM
0 8 * * * /usr/local/bin/pb --start "2025-01-01" --end "2025-12-31" --interval 86400 | head -1 | mail -s "Year Progress" user@example.com
```

### Example 11: With System Notifications
Integrate with desktop notifications:

```bash
#!/bin/bash
# notify_progress.sh

pb --start "2025-01-27 14:00:00" --end "1h" --interval 300 | while read -r line; do
    percentage=$(echo "$line" | grep -o '[0-9]*\.[0-9]*%')
    
    case $percentage in
        "25.0%")
            notify-send "Progress Update" "25% complete - keep going!"
            ;;
        "50.0%")
            notify-send "Progress Update" "50% complete - halfway there!"
            ;;
        "75.0%")
            notify-send "Progress Update" "75% complete - almost done!"
            ;;
        "100.0%")
            notify-send "Progress Complete" "Task finished!"
            ;;
    esac
done
```

### Example 12: Web Dashboard Integration
Send progress updates to a web service:

```bash
#!/bin/bash
# web_integration.sh

API_ENDPOINT="https://api.example.com/progress"
API_KEY="your-api-key"

pb --start "2025-01-27 09:00:00" --end "8h" --interval 600 | while read -r line; do
    percentage=$(echo "$line" | grep -o '[0-9]*\.[0-9]*%' | sed 's/%//')
    
    # Send to API
    curl -X POST "$API_ENDPOINT" \
         -H "Authorization: Bearer $API_KEY" \
         -H "Content-Type: application/json" \
         -d "{\"progress\": $percentage, \"timestamp\": \"$(date -Iseconds)\"}"
done
```

## Creative Use Cases

### Example 13: Pomodoro Technique
Implement the Pomodoro Technique with pb:

```bash
#!/bin/bash
# pomodoro.sh

for i in {1..4}; do
    echo "Starting Pomodoro session $i/4"
    pb --start "$(date '+%Y-%m-%d %H:%M:%S')" --end "25m" --interval 60
    
    if [ $i -lt 4 ]; then
        echo "Break time! (5 minutes)"
        pb --start "$(date '+%Y-%m-%d %H:%M:%S')" --end "5m" --interval 30
    fi
done

echo "Long break time! (30 minutes)"
pb --start "$(date '+%Y-%m-%d %H:%M:%S')" --end "30m" --interval 60
```

### Example 14: Workout Timer
Track workout sessions:

```bash
#!/bin/bash
# workout_timer.sh

exercises=("Push-ups" "Squats" "Planks" "Jumping Jacks")
duration="45s"
rest="15s"

for exercise in "${exercises[@]}"; do
    echo "Starting: $exercise"
    pb --start "$(date '+%Y-%m-%d %H:%M:%S')" --end "$duration" --interval 1
    
    echo "Rest period"
    pb --start "$(date '+%Y-%m-%d %H:%M:%S')" --end "$rest" --interval 1
done

echo "Workout complete!"
```

### Example 15: Reading Session
Track reading time with chapter breaks:

```bash
#!/bin/bash
# reading_session.sh

BOOK_TITLE="The Great Gatsby"
CHAPTER=1
READING_TIME="30m"

echo "Reading session: $BOOK_TITLE - Chapter $CHAPTER"
pb --start "$(date '+%Y-%m-%d %H:%M:%S')" --end "$READING_TIME" --interval 120

echo "Chapter $CHAPTER completed! Take a break."
```

## Troubleshooting Examples

### Example 16: Debugging Time Issues
Test different time formats:

```bash
#!/bin/bash
# test_time_formats.sh

echo "Testing date format..."
pb --start "2025-01-27" --end "2025-01-28" --interval 5 &
PID1=$!
sleep 10
kill $PID1

echo "Testing datetime format..."
pb --start "2025-01-27 14:00:00" --end "2025-01-27 15:00:00" --interval 5 &
PID2=$!
sleep 10
kill $PID2

echo "Testing relative time..."
pb --start "2025-01-27 14:00:00" --end "1h" --interval 5 &
PID3=$!
sleep 10
kill $PID3

echo "All formats tested successfully!"
```

### Example 17: Performance Testing
Test pb performance with different intervals:

```bash
#!/bin/bash
# performance_test.sh

intervals=(1 5 10 30 60)

for interval in "${intervals[@]}"; do
    echo "Testing interval: ${interval}s"
    
    # Measure CPU usage
    pb --start "2025-01-27 14:00:00" --end "5m" --interval "$interval" &
    PB_PID=$!
    
    # Monitor for 30 seconds
    top -p $PB_PID -n 6 -d 5 | grep pb > "performance_${interval}s.log" &
    TOP_PID=$!
    
    sleep 30
    kill $PB_PID $TOP_PID 2>/dev/null
    
    echo "Results saved to performance_${interval}s.log"
done
```

## Tips and Best Practices

### Choosing Update Intervals
- **1-10 seconds**: Short-term activities (cooking, workouts)
- **30-60 seconds**: Medium-term activities (meetings, study sessions)
- **5-15 minutes**: Long-term activities (work days, projects)
- **1+ hours**: Very long-term tracking (project deadlines, countdowns)

### Error Handling in Scripts
Always include error handling:

```bash
#!/bin/bash
set -e  # Exit on error

start_time="2025-01-27 09:00:00"
end_time="2025-01-27 17:00:00"

if ! pb --start "$start_time" --end "$end_time" 2>/dev/null; then
    echo "Error: Failed to start progress bar"
    echo "Check time format and system clock"
    exit 1
fi
```

### Resource Management
For long-running pb instances:

```bash
# Set resource limits
ulimit -v 102400  # Limit virtual memory to 100MB
ulimit -t 86400   # Limit CPU time to 24 hours

pb --start "..." --end "..." --interval 3600
```

### Log Rotation
For continuous logging:

```bash
#!/bin/bash
# With log rotation
pb --start "..." --end "..." --interval 300 | \
    while read -r line; do
        echo "$(date -Iseconds): $line" >> progress.log
        
        # Rotate log if it gets too large
        if [ $(stat -f%z progress.log 2>/dev/null || stat -c%s progress.log) -gt 1048576 ]; then
            mv progress.log progress.log.old
        fi
    done
```

These examples demonstrate the flexibility and power of pb for various time-tracking scenarios. Adapt them to your specific needs and use cases.