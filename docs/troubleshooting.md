# Troubleshooting Guide

This guide helps you resolve common issues when using or developing pb (progress bar tool).

## Table of Contents

- [Installation Issues](#installation-issues)
- [Runtime Issues](#runtime-issues)
- [Display and Terminal Issues](#display-and-terminal-issues)
- [Time Parsing Issues](#time-parsing-issues)
- [Performance Issues](#performance-issues)
- [Development Issues](#development-issues)
- [Error Messages](#error-messages)
- [FAQ](#faq)

## Installation Issues

### "pb: command not found"

**Symptoms**: Shell reports that pb command is not found.

**Causes**:
- pb is not installed
- pb is not in system PATH
- Installation was not completed successfully

**Solutions**:

1. **Verify installation**:
   ```bash
   # Check if pb exists in common locations
   ls -la /usr/local/bin/pb
   ls -la ~/.cargo/bin/pb
   ls -la $(which pb)
   ```

2. **Add to PATH**:
   ```bash
   # Temporarily add to PATH
   export PATH=$PATH:/path/to/pb/directory
   
   # Permanently add to PATH (add to ~/.bashrc or ~/.zshrc)
   echo 'export PATH=$PATH:/usr/local/bin' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Reinstall pb**:
   ```bash
   # Using cargo
   cargo install --git https://github.com/matsuokashuhei/pb.git
   
   # Or download binary manually
   curl -L -o pb https://github.com/matsuokashuhei/pb/releases/latest/download/pb-linux-x86_64
   chmod +x pb
   sudo mv pb /usr/local/bin/
   ```

### "Permission denied" when running pb

**Symptoms**: `bash: ./pb: Permission denied` or similar.

**Cause**: The binary doesn't have execute permissions.

**Solution**:
```bash
# Add execute permission
chmod +x /path/to/pb

# If pb is in /usr/local/bin/
sudo chmod +x /usr/local/bin/pb
```

### Binary won't run: "No such file or directory"

**Symptoms**: Error occurs even though the file exists.

**Causes**:
- Wrong architecture (e.g., ARM binary on x86_64 system)
- Missing dynamic libraries
- Corrupted download

**Solutions**:

1. **Check architecture compatibility**:
   ```bash
   # Check your system architecture
   uname -m
   
   # Check binary architecture
   file /path/to/pb
   ```

2. **Install missing libraries** (Linux):
   ```bash
   # Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install libc6 libgcc-s1
   
   # CentOS/RHEL
   sudo yum install glibc libgcc
   ```

3. **Re-download the binary**:
   ```bash
   # Remove and re-download
   rm /path/to/pb
   curl -L -o pb https://github.com/matsuokashuhei/pb/releases/latest/download/pb-linux-x86_64
   chmod +x pb
   ```

### Windows installation issues

**Symptoms**: Various issues on Windows systems.

**Solutions**:

1. **Windows Defender blocking**:
   - Add pb.exe to Windows Defender exclusions
   - Download from official GitHub releases only

2. **PATH not updated**:
   ```powershell
   # Add to PATH in PowerShell (as Administrator)
   $env:PATH += ";C:\path\to\pb\directory"
   [Environment]::SetEnvironmentVariable("PATH", $env:PATH, [EnvironmentVariableTarget]::Machine)
   ```

3. **Missing Visual C++ Redistributable**:
   - Download and install Microsoft Visual C++ Redistributable

## Runtime Issues

### "Error parsing start/end time"

**Symptoms**: pb exits with time parsing error messages.

**Common formats and solutions**:

1. **Incorrect date format**:
   ```bash
   # Wrong
   pb --start "01/27/2025" --end "01/28/2025"
   
   # Correct
   pb --start "2025-01-27" --end "2025-01-28"
   ```

2. **Incorrect datetime format**:
   ```bash
   # Wrong
   pb --start "2025-01-27 9:00:00 AM" --end "2025-01-27 5:00:00 PM"
   
   # Correct
   pb --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"
   ```

3. **Invalid relative time**:
   ```bash
   # Wrong
   pb --start "2025-01-27 09:00:00" --end "eight hours"
   
   # Correct
   pb --start "2025-01-27 09:00:00" --end "8h"
   ```

**Supported formats**:
- Date: `YYYY-MM-DD` (e.g., `2025-01-27`)
- Datetime: `YYYY-MM-DD HH:MM:SS` (e.g., `2025-01-27 14:30:00`)
- Relative: `Nh`, `Nm`, `Ns`, `Nd` (e.g., `2h`, `30m`, `45s`, `1d`)

### "End time must be after start time"

**Symptoms**: pb reports that end time is before start time.

**Causes**:
- Incorrect date/time values
- Timezone confusion
- Misunderstanding of relative time

**Solutions**:

1. **Check time values**:
   ```bash
   # Make sure end time is later than start time
   pb --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"  # Correct
   pb --start "2025-01-27 17:00:00" --end "2025-01-27 09:00:00"  # Wrong
   ```

2. **Use relative time correctly**:
   ```bash
   # Relative time is added to start time
   pb --start "2025-01-27 09:00:00" --end "8h"  # End: 17:00:00
   ```

### Progress bar not updating

**Symptoms**: Progress bar appears but doesn't change over time.

**Causes**:
- Update interval too long
- Time range already completed
- Terminal buffering issues

**Solutions**:

1. **Check time range**:
   ```bash
   # Make sure you're within the time range
   date  # Check current time
   pb --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"
   ```

2. **Reduce update interval**:
   ```bash
   # Update every 5 seconds instead of 60
   pb --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00" --interval 5
   ```

3. **Force terminal flush**:
   ```bash
   # Use unbuffered output
   pb --start "..." --end "..." | cat
   ```

## Display and Terminal Issues

### No colors in progress bar

**Symptoms**: Progress bar appears without colors, all in white/default color.

**Causes**:
- Terminal doesn't support colors
- Colors disabled by environment
- Output being redirected

**Solutions**:

1. **Check terminal color support**:
   ```bash
   # Check if terminal supports colors
   echo $TERM
   tput colors
   
   # Test color output
   echo -e "\033[31mRed\033[0m \033[32mGreen\033[0m \033[33mYellow\033[0m"
   ```

2. **Use a color-capable terminal**:
   - **Linux**: Use GNOME Terminal, Konsole, or Alacritty
   - **macOS**: Use Terminal.app or iTerm2
   - **Windows**: Use Windows Terminal or ConEmu

3. **Check environment variables**:
   ```bash
   # These variables might disable colors
   echo $NO_COLOR
   echo $TERM
   
   # Unset if they're causing issues
   unset NO_COLOR
   ```

### Progress bar formatting issues

**Symptoms**: Progress bar appears malformed or with weird characters.

**Causes**:
- Terminal encoding issues
- Font doesn't support Unicode characters
- Terminal width too narrow

**Solutions**:

1. **Check terminal encoding**:
   ```bash
   # Ensure UTF-8 encoding
   echo $LANG
   export LANG=en_US.UTF-8
   ```

2. **Use a Unicode-capable font**:
   - Install fonts like DejaVu Sans Mono, Source Code Pro, or Fira Code
   - Avoid older bitmap fonts

3. **Increase terminal width**:
   ```bash
   # Make terminal window wider (at least 80 characters)
   # Or use a smaller update interval for shorter bars
   ```

### Progress bar overwriting text

**Symptoms**: Progress bar overwrites previous terminal output.

**Cause**: Terminal in raw mode affecting output behavior.

**Solutions**:

1. **Run in non-TTY mode**:
   ```bash
   # Pipe output to get line-by-line updates
   pb --start "..." --end "..." | cat
   ```

2. **Use screen or tmux**:
   ```bash
   # Run inside screen/tmux for better terminal handling
   screen -S pb
   pb --start "..." --end "..."
   ```

## Time Parsing Issues

### Daylight Saving Time problems

**Symptoms**: Unexpected time calculations during DST transitions.

**Cause**: pb uses local system time, which includes DST adjustments.

**Solutions**:

1. **Use UTC times**:
   ```bash
   # Convert to UTC before using pb
   date -u
   ```

2. **Be aware of DST transitions**:
   - Spring forward: 2 AM becomes 3 AM (1 hour lost)
   - Fall back: 2 AM becomes 1 AM (1 hour gained)

3. **Use date-only format** to avoid time-of-day issues:
   ```bash
   pb --start "2025-01-27" --end "2025-01-28"
   ```

### Leap year/leap second issues

**Symptoms**: Unexpected behavior on February 29 or during leap seconds.

**Solutions**:

1. **Verify leap year dates**:
   ```bash
   # 2024 is a leap year, 2025 is not
   pb --start "2024-02-29" --end "2024-03-01"  # Valid
   pb --start "2025-02-29" --end "2025-03-01"  # Invalid
   ```

2. **Use shorter time ranges** during leap second periods (rare).

### Timezone confusion

**Symptoms**: Progress shows unexpected values due to timezone differences.

**Cause**: System timezone doesn't match expected timezone.

**Solutions**:

1. **Check system timezone**:
   ```bash
   # Check current timezone
   timedatectl
   date
   
   # Set timezone if needed
   sudo timedatectl set-timezone America/New_York
   ```

2. **Convert times to local timezone**:
   ```bash
   # Use local time consistently
   date "+%Y-%m-%d %H:%M:%S"
   ```

## Performance Issues

### High CPU usage

**Symptoms**: pb consumes significant CPU resources.

**Causes**:
- Very short update interval
- Terminal rendering issues
- System performance problems

**Solutions**:

1. **Increase update interval**:
   ```bash
   # Use longer intervals for reduced CPU usage
   pb --start "..." --end "..." --interval 60  # Update every minute
   ```

2. **Check system resources**:
   ```bash
   # Monitor resource usage
   top -p $(pgrep pb)
   htop
   ```

3. **Use non-TTY mode** for reduced overhead:
   ```bash
   pb --start "..." --end "..." > progress.log &
   tail -f progress.log
   ```

### Memory issues

**Symptoms**: pb uses excessive memory or system becomes slow.

**Causes**:
- Memory leak (rare)
- Large terminal buffer
- System memory pressure

**Solutions**:

1. **Monitor memory usage**:
   ```bash
   # Check memory usage
   ps aux | grep pb
   ```

2. **Restart pb** if memory usage grows unexpectedly:
   ```bash
   # Kill and restart
   pkill pb
   pb --start "..." --end "..."
   ```

3. **Clear terminal buffer**:
   ```bash
   # Clear terminal history
   clear
   reset
   ```

## Development Issues

### Build failures

**Symptoms**: `cargo build` fails with compilation errors.

**Solutions**:

1. **Update Rust toolchain**:
   ```bash
   rustup update
   rustc --version  # Should be 1.70+
   ```

2. **Clean build cache**:
   ```bash
   cargo clean
   cargo build
   ```

3. **Check dependencies**:
   ```bash
   cargo update
   cargo tree
   ```

### Test failures

**Symptoms**: `cargo test` fails with test errors.

**Solutions**:

1. **Run tests with verbose output**:
   ```bash
   cargo test -- --nocapture
   RUST_BACKTRACE=1 cargo test
   ```

2. **Run specific test**:
   ```bash
   cargo test test_name -- --exact
   ```

3. **Check for timing issues**:
   ```bash
   # Run tests single-threaded
   cargo test -- --test-threads=1
   ```

### Docker issues

**Symptoms**: Docker-based development not working.

**Solutions**:

1. **Rebuild Docker image**:
   ```bash
   docker build --no-cache .
   ```

2. **Check Docker daemon**:
   ```bash
   docker ps
   docker system info
   ```

3. **Fix volume permissions**:
   ```bash
   # Linux: Fix ownership
   sudo chown -R $USER:$USER .
   ```

## Error Messages

### Common Error Messages and Solutions

#### "Invalid time format: [input]"
```bash
# Error
pb --start "invalid" --end "2025-01-27"

# Solution: Use valid format
pb --start "2025-01-27" --end "2025-01-28"
```

#### "End time must be after start time"
```bash
# Error
pb --start "2025-01-27 17:00:00" --end "2025-01-27 09:00:00"

# Solution: Ensure proper time order
pb --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"
```

#### "Failed to parse relative time"
```bash
# Error
pb --start "2025-01-27" --end "invalid"

# Solution: Use valid relative format
pb --start "2025-01-27" --end "1d"  # or "24h", "1440m", etc.
```

#### "Terminal error: ..."
```bash
# Usually indicates terminal capability issues
# Try running in non-TTY mode:
pb --start "..." --end "..." | cat
```

## FAQ

### Q: Can I pause and resume pb?
**A**: No, pb tracks real time continuously. To "pause", stop pb (Ctrl+C) and restart with adjusted times.

### Q: Why does pb show >100%?
**A**: This happens when the current time exceeds the end time. The progress bar turns red to indicate overtime.

### Q: Can I use pb for past time periods?
**A**: Yes, but it will immediately show 100% completion since the time period has elapsed.

### Q: Does pb work in scripts?
**A**: Yes, pb works well in scripts. Use pipe output for line-by-line updates:
```bash
pb --start "..." --end "..." | while read line; do
    echo "Progress: $line"
done
```

### Q: How accurate is pb?
**A**: pb is accurate to the second level, limited by your system clock and the update interval.

### Q: Can I run multiple pb instances?
**A**: Yes, each pb instance runs independently. Use different terminals or background processes.

### Q: Why doesn't pb work in my IDE terminal?
**A**: Some IDE terminals have limited ANSI support. Try:
- Using the system terminal
- Running with pipe output: `pb ... | cat`
- Checking IDE terminal settings

### Q: How do I report a bug?
**A**: 
1. Check this troubleshooting guide first
2. Search [existing issues](https://github.com/matsuokashuhei/pb/issues)
3. Create a new issue with:
   - Operating system and version
   - pb version (`pb --version`)
   - Command that failed
   - Error message
   - Terminal type

### Q: How do I contribute fixes?
**A**: See the [Development Guide](development_guide.md) for contribution guidelines.

## Getting Additional Help

If your issue isn't covered here:

1. **Check the documentation**:
   - [User Guide](user_guide.md)
   - [Installation Guide](installation.md)
   - [Development Guide](development_guide.md)

2. **Search existing issues**: [GitHub Issues](https://github.com/matsuokashuhei/pb/issues)

3. **Create a new issue** with detailed information:
   - Operating system and version
   - pb version (`pb --version`)
   - Complete command that fails
   - Full error message
   - Steps to reproduce

4. **Join discussions**: [GitHub Discussions](https://github.com/matsuokashuhei/pb/discussions)

Remember to include as much detail as possible when reporting issues to help maintainers diagnose and fix problems quickly.