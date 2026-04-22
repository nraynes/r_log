# R_Log

A simple logging crate with multiple loggers for different logging systems as well as the ability to make a custom logger. Everything is fully tested with unit tests to ensure reliability.

## Current Loggers

These are the loggers that are currently available.

### SysLog

The SysLog system uses 8 logging levels that are as such:

- (0) **Emergency**: System is unusable
- (0) **Alert**: Action must be taken immediately
- (0) **Critical**: Critical conditions
- (0) **Error**: Error conditions
- (0) **Warning**: Warning conditions
- (0) **Notification**: Normal but significant condition
- (0) **Info**: Informational messages
- (0) **Debug**: Debugging messages.

## Custom Loggers

You can make a custom logger by taking levels from various systems or using a custom one and instantiating a logger with them. This feature is not available yet, and more documentation will be added when it is available.

## Examples

    use r_log::Logger;

    let logger = Logger::new();

    logger.info("This is some information.");
