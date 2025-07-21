# Statement of Work: Phase 3 - Relative Time Parsing Implementation

## Project Overview

### Task Name
Phase 3: Relative Time Parsing Implementation

### Task Description
Implement comprehensive relative time format parsing functionality for formats like `30m`, `2h`, `1d` using regex patterns and convert to absolute timestamps based on current time.

### Dependencies
- Issue #1: Project structure (Completed)
- Issue #3: Error handling system (Completed)
- Issue #5: Date format parsing (Completed)
- `regex` crate dependency (Already configured)
- `chrono` crate dependency (Already configured)

## Scope of Work

### Core Requirements

#### 1. Relative Time Parsing Function
**Function Signature**: `pub fn parse_relative_time(input: &str, base_time: NaiveDateTime) -> Result<NaiveDateTime, PbError>`

**Functionality**:
- Parse relative time strings in formats: `30m` (minutes), `2h` (hours), `1d` (days)
- Use regex pattern `^(\d+)([mhd])$` for parsing
- Convert relative times to absolute timestamps based on provided base time
- Support reasonable ranges (1-999 for each unit)
- Return appropriate error for invalid formats

#### 2. Error Handling
**Error Cases**:
- Invalid relative time format (wrong patterns, unsupported units)
- Out of range values (0 or excessively large values)
- Edge cases (overflow scenarios, boundary conditions)

**Error Messages**:
- Clear, user-friendly error descriptions
- Include the problematic input in error message
- Utilize existing `PbError::InvalidRelativeTimeFormat` error type

#### 3. Unit Conversion
**Supported Units**:
- `m`: Minutes → seconds conversion (×60)
- `h`: Hours → seconds conversion (×3600)
- `d`: Days → seconds conversion (×86400)

**Range Validation**:
- Minimum value: 1 (reject 0 values)
- Maximum value: 999 (reasonable upper bound)
- Handle integer overflow scenarios

#### 4. Integration with Existing Time Parser
**Requirements**:
- Seamless integration with existing `parse_date` function
- Consistent error handling patterns
- Maintain existing API compatibility
- Follow established code patterns and documentation style

## Implementation Details

### Code Structure
```rust
// In src/time_parser.rs
use regex::Regex;
use chrono::{NaiveDateTime, Duration};
use crate::error::PbError;

pub fn parse_relative_time(input: &str, base_time: NaiveDateTime) -> Result<NaiveDateTime, PbError> {
    let re = Regex::new(r"^(\d+)([mhd])$").unwrap();
    
    if let Some(captures) = re.captures(input) {
        let amount: i64 = captures[1].parse()
            .map_err(|_| PbError::InvalidRelativeTimeFormat { input: input.to_string() })?;
        let unit = &captures[2];
        
        // Validate range (1-999)
        if amount < 1 || amount > 999 {
            return Err(PbError::InvalidRelativeTimeFormat { input: input.to_string() });
        }
        
        let seconds = match unit {
            "m" => amount * 60,
            "h" => amount * 3600,
            "d" => amount * 86400,
            _ => return Err(PbError::InvalidRelativeTimeFormat { input: input.to_string() }),
        };
        
        base_time.checked_add_signed(Duration::seconds(seconds))
            .ok_or_else(|| PbError::InvalidRelativeTimeFormat { input: input.to_string() })
    } else {
        Err(PbError::InvalidRelativeTimeFormat { input: input.to_string() })
    }
}
```

### Test Cases

#### Valid Formats
- `"30m"` → 30 minutes from base time
- `"2h"` → 2 hours from base time
- `"1d"` → 1 day from base time
- `"120m"` → 120 minutes from base time
- `"24h"` → 24 hours from base time
- `"999m"`, `"999h"`, `"999d"` → maximum allowed values

#### Invalid Formats
- `"30"` → missing unit
- `"m30"` → wrong order
- `"30x"` → invalid unit
- `"30mins"` → verbose unit not supported
- `"2.5h"` → decimal not supported
- `"0m"` → zero value not allowed
- `"1000m"` → exceeds maximum range
- `"-5h"` → negative values not supported

#### Edge Cases
- Very large valid values near the limit (999d)
- Overflow scenarios (base_time + duration > max datetime)
- Empty strings and whitespace
- Unicode characters and special symbols

## Deliverables

### 1. Implementation
- [ ] `parse_relative_time` function in `src/time_parser.rs`
- [ ] Integration with existing error handling system
- [ ] Documentation with examples
- [ ] Export function through `src/lib.rs`

### 2. Comprehensive Test Suite
- [ ] Unit tests for valid relative time parsing
- [ ] Unit tests for invalid format detection
- [ ] Unit tests for range validation
- [ ] Edge case tests (overflow, boundary conditions)
- [ ] Integration tests with existing date parsing
- [ ] Performance tests for repeated parsing

### 3. Documentation Updates
- [ ] Update function documentation with examples
- [ ] Update project README if needed
- [ ] Create implementation report

### 4. Code Quality
- [ ] Follow Rust best practices
- [ ] Proper error handling with existing patterns
- [ ] Clear, readable code with comments
- [ ] Integration with existing codebase style

## Acceptance Criteria

### Functional Requirements
- [ ] Parse relative time formats `30m`, `2h`, `1d` correctly
- [ ] Use regex pattern `^(\d+)([mhd])$` for parsing
- [ ] Convert relative times to absolute timestamps based on base time
- [ ] Handle invalid relative formats gracefully with clear error messages
- [ ] Support reasonable ranges (1-999 for each unit)
- [ ] Handle edge cases (overflow, zero values, large values)

### Quality Requirements
- [ ] All tests pass (aim for 15+ test functions)
- [ ] 100% test coverage for new functions
- [ ] Zero compiler warnings
- [ ] Zero clippy warnings
- [ ] Performance: <1ms for typical parsing operations
- [ ] Documentation examples verified and working

### Integration Requirements
- [ ] Seamless integration with existing `time_parser.rs` module
- [ ] Consistent error handling with existing patterns
- [ ] Exported through public API in `lib.rs`
- [ ] Compatible with existing CLI argument handling

## Risk Assessment

### Technical Risks
1. **Integer Overflow**: Mitigated by range validation and checked arithmetic
2. **Regex Performance**: Mitigated by using efficient regex patterns
3. **DateTime Overflow**: Mitigated by using `checked_add_signed()`

### Timeline Risks
1. **Complex Edge Cases**: Mitigated by comprehensive test planning
2. **Integration Complexity**: Mitigated by following existing patterns

## Success Metrics
1. **Functionality**: All specified relative time formats working correctly
2. **Performance**: Fast parsing (<1ms typical case)
3. **Quality**: High test coverage (100%) and zero warnings
4. **Integration**: Seamless integration with existing codebase

## Estimated Timeline
- **Implementation**: 0.5 days
- **Testing**: 0.5 days
- **Documentation**: 0.5 days
- **Total**: 1.5 days

This implementation will complete the relative time parsing functionality as specified in Issue #7 and provide a solid foundation for the remaining application features.
