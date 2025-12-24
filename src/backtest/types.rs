use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub cash: f64,
    pub btc_position: f64,
    pub total_trades: u32,
}

impl Portfolio {
    pub fn new(initial_cash: f64) -> Self {
        Self {
            cash: initial_cash,
            btc_position: 0.0,
            total_trades: 0,
        }
    }

    pub fn equity(&self, btc_price: f64) -> f64 {
        self.cash + (self.btc_position * btc_price)
    }

    pub fn buy(&mut self, btc_amount: f64, price: f64, commission_bps: f64) {
        let cost = btc_amount * price;
        let commission = cost * (commission_bps / 10000.0);
        let total_cost = cost + commission;

        self.cash -= total_cost;
        self.btc_position += btc_amount;
        self.total_trades += 1;
    }

    pub fn sell(&mut self, btc_amount: f64, price: f64, commission_bps: f64) {
        let proceeds = btc_amount * price;
        let commission = proceeds * (commission_bps / 10000.0);
        let net_proceeds = proceeds - commission;

        self.cash += net_proceeds;
        self.btc_position -= btc_amount;
        self.total_trades += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionModel {
    pub commission_bps: f64,
    pub slippage_bps: f64,
}

impl ExecutionModel {
    pub fn new(commission_bps: f64, slippage_bps: f64) -> Self {
        Self {
            commission_bps,
            slippage_bps,
        }
    }

    pub fn execute_market_buy(&self, price: f64) -> f64 {
        price * (1.0 + self.slippage_bps / 10000.0)
    }

    pub fn execute_market_sell(&self, price: f64) -> f64 {
        price * (1.0 - self.slippage_bps / 10000.0)
    }
}
