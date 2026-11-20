# SigmaOS-actions Component Wiki

## Overview
SigmaOS-actions provides reusable GitHub Actions for SigmaOS repositories. It demonstrates the CI/CD automation patterns and component testing frameworks within the SigmaOS ecosystem, enabling consistent build, test, and deployment pipelines across all SigmaOS components.

## Principles
- Single Responsibility
- CI/CD Automation
- Fail-Fast on Pipeline Errors
- Deterministic Pipeline Initialization
- Clear Abstraction Over GitHub Actions APIs
- Composition over Inheritance for action workflows

## Architecture
SigmaOS-actions follows a GitHub Actions automation pattern:
- **Workflow Templates**: Provides standardized CI/CD workflow templates for all SigmaOS components
- **Testing Frameworks**: Integrates AAA test pattern execution across component repos
- **Artifact Management**: Handles test artifacts, build logs, and deployment packages
- **Pipeline Orchestration**: Coordinates multi-stage CI/CD pipelines for component validation

## API
The component provides the following public APIs (Actions context):
- `workflow::trigger::on(push, pull_request)` - Define pipeline triggers
- `workflow::job::run(name, steps)` - Execute a named job with steps
- `workflow::artifact::upload(name, path)` - Upload pipeline artifacts
- `workflow::matrix::run(dimensions, steps)` - Run matrix testing across configurations
- Error types for pipeline failures or misconfigured workflows

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern, executed within GitHub Actions workflows:
1. **Arrange**: Set up the test environment with necessary dependencies
2. **Act**: Execute the code under test (build, test, lint steps)
3. **Assert**: Verify the expected pipeline results (success/failure conditions)

Example within a GitHub Actions workflow:
```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - name: Arrange
        uses: actions/checkout@v3
      - name: Act
        run: cargo test --offline
      - name: Assert
        run: echo "All tests passed"
```

## Development
- Follows GitHub Actions best practices with YAML workflow definitions
- Pipeline initialization is best-effort; gracefully degrades on missing dependencies
- Comprehensive error types for all pipeline failure scenarios
- Workflow templates provided for standard SigmaOS component testing
- Integration tests for full CI/CD pipeline execution across component repos

## TODO
- Add support for additional pipeline stages (build, deploy, release)
- Expand workflow templates for different component types
- Add caching strategies for faster pipeline execution
- Create integration tests with actual SigmaOS component repositories
- Implement pipeline analytics and reporting features