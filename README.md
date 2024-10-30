# Capital Gains Tax Calculator

This project implements a tax calculator for stock portfolio operations in Rust. It processes a series of buy and sell operations and calculates the applicable taxes based on provided tax regulations.

## Technical Decisions and Architecture

1. **Language Choice**: Rust was chosen for its performance, memory safety, and strong type system.

2. **Data Structures**: 
   - [`State`](src/simulation.rs): Represents the current state of the portfolio.
   - [`Op`](src/simulation.rs): Represents individual buy/sell operations.
   - [`Tax`](src/simulation.rs): Represents the calculated tax for each operation.

3. **Algorithm**: The program uses a state-based approach, updating the portfolio state after each operation and calculating taxes accordingly.

4. **Error Handling**: The code uses Rust's `Result` type for error handling, ensuring robustness and clear error reporting.

5. **Testing**: Comprehensive unit tests are included to verify the correctness of tax calculations for various scenarios.

## Libraries Used

1. **serde**: Used for JSON serialization/deserialization, simplifying input parsing.
2. **serde_json**: Provides JSON-specific functionalities for serde.

These libraries were chosen for their reliability, performance, and wide adoption in the Rust ecosystem.

## Compilation and Execution

1. Ensure you have Rust and Cargo installed on your system. If not, you can install them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. Navigate to the project directory.
4. To compile and build the project:

   ```
   cargo build --release
   ```

   This will build the project in release mode. The resulting binary will be in the `./target/release` directory.

5. To run the compiled binary:

   ```
   ./target/release/capital-gains < fixtures.json
   ```

   Replace `fixtures.json` with the path to your input file.

6. Alternatively, you can compile and run the project in one step:

   ```
   cargo run --release -- < fixtures.json
   ```

7. To clean up built files:

   ```
   cargo clean
   ```

Note: Cargo ensures a consistent build environment across different systems, so there's no need for Docker in this setup.

## Running Tests

To run the test suite:

   ```
   cargo test
   ```

## Additional Notes

- The project follows Rust best practices and idioms.
- The code is thoroughly documented with comments explaining the purpose and functionality of each component.
- The tax calculation logic is based on the provided rules, including handling of accumulated losses and tax-free thresholds.
- The solution is designed to be easily extendable for additional features or rule changes.


For any questions or clarifications, please refer to the code comments or reach out to Pedro.