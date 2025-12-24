use serde::{Deserialize, Serialize};

/// Risk management limits and controls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimits {
    /// Maximum position size as percentage of equity
    pub max_position_pct: f64,
    
    /// Maximum portfolio heat (total exposure / equity)
    pub max_portfolio_heat: f64,
    
    /// Maximum drawdown from peak before stopping trading
    pub max_drawdown_threshold: f64,
    
    /// Maximum number of concurrent positions (for multi-asset)
    pub max_concurrent_positions: usize,
    
    /// Maximum trades per day (0 = unlimited)
    pub max_trades_per_day: usize,
    
    /// Minimum time between trades (in bars)
    pub min_bars_between_trades: usize,
}

impl RiskLimits {
    pub fn new() -> Self {
        Self {
            max_position_pct: 100.0,      // 100% = all-in allowed
            max_portfolio_heat: 1.0,       // 1.0 = 100% exposure allowed
            max_drawdown_threshold: 0.30,  // Stop at -30% drawdown
            max_concurrent_positions: 1,   // Single asset for now
            max_trades_per_day: 0,         // Unlimited
            min_bars_between_trades: 0,    // No minimum
        }
    }
    
    /// Conservative risk limits
    pub fn conservative() -> Self {
        Self {
            max_position_pct: 50.0,
            max_portfolio_heat: 0.5,
            max_drawdown_threshold: 0.20,
            max_concurrent_positions: 1,
            max_trades_per_day: 2,
            min_bars_between_trades: 5,
        }
    }
    
    /// Aggressive risk limits
    pub fn aggressive() -> Self {
        Self {
            max_position_pct: 100.0,
            max_portfolio_heat: 1.0,
            max_drawdown_threshold: 0.50,
            max_concurrent_positions: 1,
            max_trades_per_day: 0,
            min_bars_between_trades: 0,
        }
    }
    
    /// Check if position size exceeds limits
    pub fn check_position_size(&self, position_value: f64, equity: f64) -> bool {
        let position_pct = (position_value / equity) * 100.0;
        position_pct <= self.max_position_pct
    }
    
    /// Check if portfolio heat is within limits
    pub fn check_portfolio_heat(&self, total_exposure: f64, equity: f64) -> bool {
        let heat = total_exposure / equity;
        heat <= self.max_portfolio_heat
    }
    
    /// Check if drawdown threshold exceeded
    pub fn check_drawdown(&self, current_equity: f64, peak_equity: f64) -> bool {
        if peak_equity == 0.0 {
            return true;
        }
        
        let drawdown = (current_equity - peak_equity) / peak_equity;
        drawdown.abs() < self.max_drawdown_threshold
    }
    
    /// Check if can trade (time-based limits)
    pub fn can_trade(
        &self,
        bars_since_last_trade: usize,
        trades_today: usize,
    ) -> bool {
        // Check minimum bars between trades
        if bars_since_last_trade < self.min_bars_between_trades {
            return false;
        }
        
        // Check max trades per day (0 = unlimited)
        if self.max_trades_per_day > 0 && trades_today >= self.max_trades_per_day {
            return false;
        }
        
        true
    }
}

impl Default for RiskLimits {
    fn default() -> Self {
        Self::new()
    }
}

/// Track risk metrics during backtest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    pub peak_equity: f64,
    pub current_drawdown: f64,
    pub max_drawdown_hit: f64,
    pub total_exposure: f64,
    pub trades_today: usize,
    pub bars_since_last_trade: usize,
    pub risk_limit_violations: usize,
}

impl RiskMetrics {
    pub fn new(initial_equity: f64) -> Self {
        Self {
            peak_equity: initial_equity,
            current_drawdown: 0.0,
            max_drawdown_hit: 0.0,
            total_exposure: 0.0,
            trades_today: 0,
            bars_since_last_trade: 0,
            risk_limit_violations: 0,
        }
    }
    
    pub fn update(&mut self, equity: f64, exposure: f64) {
        if equity > self.peak_equity {
            self.peak_equity = equity;
        }
        
        self.current_drawdown = (equity - self.peak_equity) / self.peak_equity;
        
        if self.current_drawdown < self.max_drawdown_hit {
            self.max_drawdown_hit = self.current_drawdown;
        }
        
        self.total_exposure = exposure;
        self.bars_since_last_trade += 1;
    }
    
    pub fn on_trade(&mut self) {
        self.trades_today += 1;
        self.bars_since_last_trade = 0;
    }
    
    pub fn on_new_day(&mut self) {
        self.trades_today = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_size_limit() {
        let limits = RiskLimits::new();
        
        // 50k position on 100k equity = 50%
        assert!(limits.check_position_size(50000.0, 100000.0));
        
        // 150k position on 100k equity = 150%
        assert!(!limits.check_position_size(150000.0, 100000.0));
    }

    #[test]
    fn test_drawdown_limit() {
        let limits = RiskLimits::new();
        
        // 20% drawdown on 30% threshold = OK
        assert!(limits.check_drawdown(80000.0, 100000.0));
        
        // 40% drawdown on 30% threshold = STOP
        assert!(!limits.check_drawdown(60000.0, 100000.0));
    }

    #[test]
    fn test_can_trade() {
        let mut limits = RiskLimits::new();
        limits.max_trades_per_day = 2;
        limits.min_bars_between_trades = 5;
        
        // Too soon since last trade
        assert!(!limits.can_trade(3, 0));
        
        // Enough time, but too many trades today
        assert!(!limits.can_trade(10, 2));
        
        // OK to trade
        assert!(limits.can_trade(10, 1));
    }
}
