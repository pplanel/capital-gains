use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The upper limit for non taxable operations.
const TAXABLE_AMOUNT: f64 = 20000.00;

const TAX_AMOUNT: f64 = 0.2;

/// Represents the current state of a stock portfolio.
///
/// This struct holds information about the total number of shares,
/// the weighted average price of the shares, and any accumulated losses.
#[derive(Debug, Default, Clone)]
pub struct State {
    /// The total number of shares in the portfolio.
    pub total_shares: usize,
    /// The weighted average price of all shares in the portfolio.
    pub weighted_avarage: f64,
    /// The total accumulated loss, if any.
    pub accumulated_loss: f64,
}

impl State {
    /// Creates a new `State` instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `total_shares` - The total number of shares in the portfolio.
    /// * `weighted_avarage` - The weighted average price of all shares.
    /// * `accumulated_loss` - The total accumulated loss.
    ///
    /// # Returns
    ///
    /// A new `State` instance.
    pub fn new(total_shares: usize, weighted_avarage: f64, accumulated_loss: f64) -> Self {
        Self {
            total_shares,
            weighted_avarage,
            accumulated_loss,
        }
    }

    /// Handles a buy operation and computes the resulting state and tax.
    ///
    /// # Arguments
    ///
    /// * `op` - The buy operation to be handled.
    ///
    /// # Returns
    ///
    /// A tuple containing the new `State` after the buy operation and the `Tax` (always default for buy operations).
    pub fn handle_buy(&self, op: &Op) -> (Self, Tax) {
        let new_weighted_avarage = calculate_weighted_avarage(
            self.total_shares,
            self.weighted_avarage,
            op.quantity,
            op.unit_cost,
        );
        let new_state = State::new(
            self.total_shares + op.quantity,
            new_weighted_avarage,
            self.accumulated_loss,
        );
        (new_state, Tax::default())
    }

    /// Handles a sell operation and computes the resulting state and tax.
    ///
    /// # Arguments
    ///
    /// * `op` - The sell operation to be handled.
    ///
    /// # Returns
    ///
    /// A tuple containing the new `State` after the sell operation and the `Tax`.
    pub fn handle_sell(&self, op: &Op) -> (Self, Tax) {
        let profit = (op.unit_cost - self.weighted_avarage) * op.quantity as f64;

        if op.total_value() <= TAXABLE_AMOUNT && profit > 0.0 {
            return (self.clone(), Tax::default());
        }

        if profit.is_sign_negative() {
            let new_state = State::new(
                self.total_shares - op.quantity,
                self.weighted_avarage,
                self.accumulated_loss + profit.abs(),
            );

            return (new_state, Tax::default());
        }

        let net_profit = (profit - self.accumulated_loss).max(0.0);
        let tax = (net_profit * TAX_AMOUNT).round();
        let new_accumulated_loss = (self.accumulated_loss - profit).max(0.0);
        let new_state = State::new(
            self.total_shares - op.quantity,
            self.weighted_avarage,
            new_accumulated_loss,
        );
        (new_state, Tax::new(tax))
    }
}

/// Represents the type of operation performed on the portfolio.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    /// Represents a buy operation.
    Buy,
    /// Represents a sell operation.
    Sell,
}

impl From<&'_ str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "buy" => Operation::Buy,
            "sell" => Operation::Sell,
            _ => panic!("Invalid operation"),
        }
    }
}

/// Represents a single operation performed on the portfolio.
#[derive(Debug, Deserialize)]
pub struct Op {
    /// The type of operation (buy or sell).
    pub operation: Operation,
    /// The unit cost of the shares in the operation.
    #[serde(rename = "unit-cost")]
    pub unit_cost: f64,
    /// The quantity of shares involved in the operation.
    pub quantity: usize,
}

impl Op {
    /// Calculates the total value of the operation.
    ///
    /// # Returns
    ///
    /// The total value as a `f64`.
    pub fn total_value(&self) -> f64 {
        self.unit_cost * self.quantity as f64
    }
}

/// Represents the tax calculated for an operation.
#[derive(Debug, Serialize, Default, PartialEq)]
pub struct Tax {
    tax: f64,
}

impl Tax {
    pub fn new(tax: f64) -> Self {
        Self { tax }
    }
}

impl Display for Tax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.tax)
    }
}

// Add these implementations
impl PartialEq<f64> for Tax {
    fn eq(&self, other: &f64) -> bool {
        self.tax == *other
    }
}

impl PartialEq<Tax> for f64 {
    fn eq(&self, other: &Tax) -> bool {
        *self == other.tax
    }
}

/// Calculates the new weighted average price after a buy operation.
///
/// # Arguments
///
/// * `total_shares` - The current total number of shares.
/// * `weighted_avarage` - The current weighted average price.
/// * `new_quantity` - The quantity of shares in the new operation.
/// * `new_unit_price` - The unit price of shares in the new operation.
///
/// # Returns
///
/// The new weighted average price as a `f64`.
pub fn calculate_weighted_avarage(
    total_shares: usize,
    weighted_avarage: f64,
    new_quantity: usize,
    new_unit_price: f64,
) -> f64 {
    let result = ((total_shares as f64 * weighted_avarage)
        + (new_quantity as f64 * new_unit_price))
        / (total_shares as f64 + new_quantity as f64);
    (result * 100.0).round() / 100.0
}

/// Computes taxes for a series of operations.
///
/// # Arguments
///
/// * `operations` - A slice of `Op` representing the series of operations.
///
/// # Returns
///
/// A vector of `Tax` instances representing the computed taxes for each operation.
pub fn compute_taxes(operations: &[Op]) -> Vec<Tax> {
    operations
        .iter()
        .scan(State::default(), |state, op| {
            let (new_state, tax) = handle_operation(state.clone(), op);
            *state = new_state;
            Some(tax)
        })
        .collect()
}

/// Handles a single operation and computes the resulting state and tax.
///
/// # Arguments
///
/// * `state` - The current `State` of the portfolio.
/// * `op` - The `Op` to be handled.
///
/// # Returns
///
/// A tuple containing the new `State` after the operation and the `Tax` for the operation.
pub fn handle_operation(state: State, op: &Op) -> (State, Tax) {
    match op.operation {
        Operation::Buy => state.handle_buy(op),
        Operation::Sell => state.handle_sell(op),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_op(operation: &str, unit_cost: f64, quantity: usize) -> Op {
        Op {
            operation: operation.into(),
            unit_cost,
            quantity,
        }
    }

    #[test]
    fn test_a() {
        let value = 22.0;
        println!("{:.2}", value);
    }

    #[test]
    fn test_calculate_weighted_average() {
        // Test case 1: Simple case
        assert_eq!(calculate_weighted_avarage(100, 10.0, 50, 20.0), 13.33);

        // Test case 2: No existing shares
        assert_eq!(calculate_weighted_avarage(0, 0.0, 100, 15.0), 15.00);

        // Test case 3: No new shares
        assert_eq!(calculate_weighted_avarage(100, 10.0, 0, 0.0), 10.00);

        // Test case 4: Large numbers
        assert_eq!(
            calculate_weighted_avarage(1_000_000, 50.0, 500_000, 60.0),
            53.33
        );

        // Test case 5: Fractional prices
        assert_eq!(calculate_weighted_avarage(200, 15.75, 100, 16.25), 15.92);
    }

    #[test]
    fn test_case_1() {
        let ops = vec![
            create_op("buy", 10.00, 100),
            create_op("sell", 15.00, 50),
            create_op("sell", 15.00, 50),
        ];
        let result = compute_taxes(&ops);
        assert_eq!(result, vec![0.00, 0.00, 0.00]);
    }

    #[test]
    fn test_case_2() {
        let ops = vec![
            create_op("buy", 10.00, 10000),
            create_op("sell", 20.00, 5000),
            create_op("sell", 5.00, 5000),
        ];
        let result = compute_taxes(&ops);
        assert_eq!(result, vec![0.00, 10000.00, 0.00]);
    }

    #[test]
    fn test_case_1_and2() {
        let cases = [
            r#"[{"operation":"buy", "unit-cost":10.00, "quantity": 100}, {"operation":"sell", "unit-cost":15.00, "quantity": 50},{"operation":"sell", "unit-cost":15.00, "quantity": 50}]"#,
            r#"[{"operation":"buy", "unit-cost":10.00, "quantity": 10000}, {"operation":"sell", "unit-cost":20.00, "quantity": 5000}, {"operation":"sell", "unit-cost":5.00, "quantity": 5000}]"#,
        ];

        let results: Vec<Vec<Tax>> = cases
            .iter()
            .map(|case| {
                let ops: Vec<Op> = serde_json::from_str(case).expect("Invalid JSON");
                compute_taxes(&ops)
            })
            .collect();

        assert_eq!(
            results,
            vec![vec![0.00, 0.00, 0.00], vec![0.00, 10000.00, 0.00]]
        );
    }

    #[test]
    fn test_case_3() {
        let ops = vec![
            create_op("buy", 10.00, 10000),
            create_op("sell", 5.00, 5000),
            create_op("sell", 20.00, 3000),
        ];
        let result = compute_taxes(&ops);
        assert_eq!(result, vec![0.00, 0.00, 1000.00]);
    }

    #[test]
    fn test_case_4() {
        let ops = vec![
            create_op("buy", 10.00, 10000),
            create_op("buy", 25.00, 5000),
            create_op("sell", 15.00, 10000),
        ];
        let result = compute_taxes(&ops);
        assert_eq!(result, vec![0.00, 0.00, 0.00]);
    }

    #[test]
    fn test_case_5() {
        let ops = vec![
            create_op("buy", 10.00, 10000),
            create_op("buy", 25.00, 5000),
            create_op("sell", 15.00, 10000),
            create_op("sell", 25.00, 5000),
        ];
        let result = compute_taxes(&ops);
        assert_eq!(result, vec![0.00, 0.00, 0.00, 10000.00]);
    }

    #[test]
    fn test_case_6() {
        let ops = vec![
            create_op("buy", 10.00, 10000),
            create_op("sell", 2.00, 5000),
            create_op("sell", 20.00, 2000),
            create_op("sell", 20.00, 2000),
            create_op("sell", 25.00, 1000),
        ];
        let result = compute_taxes(&ops);
        assert_eq!(result, vec![0.00, 0.00, 0.00, 0.00, 3000.00]);
    }

    #[test]
    fn test_case_7() {
        let ops = vec![
            create_op("buy", 10.00, 10000),
            create_op("sell", 2.00, 5000),
            create_op("sell", 20.00, 2000),
            create_op("sell", 20.00, 2000),
            create_op("sell", 25.00, 1000),
            create_op("buy", 20.00, 10000),
            create_op("sell", 15.00, 5000),
            create_op("sell", 30.00, 4350),
            create_op("sell", 30.00, 650),
        ];
        let result = compute_taxes(&ops);
        assert_eq!(
            result,
            vec![0.00, 0.00, 0.00, 0.00, 3000.00, 0.00, 0.00, 3700.00, 0.00]
        );
    }

    #[test]
    fn test_case_8() {
        let ops = vec![
            create_op("buy", 10.00, 10000),
            create_op("sell", 50.00, 10000),
            create_op("buy", 20.00, 10000),
            create_op("sell", 50.00, 10000),
        ];
        let result = compute_taxes(&ops);
        assert_eq!(result, vec![0.00, 80000.00, 0.00, 60000.00]);
    }
}
