use serde::{Deserialize, Serialize};

/// Represents a single completed trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub entry_timestamp: i64,
    pub exit_timestamp: i64,
    pub entry_price: f64,
    pub exit_price: f64,
    pub position_size: f64,
    pub pnl: f64,
    pub pnl_pct: f64,
    pub duration_bars: usize,
    pub is_win: bool,
}

impl Trade {
    pub fn new(
        entry_timestamp: i64,
        exit_timestamp: i64,
        entry_price: f64,
        exit_price: f64,
        position_size: f64,
    ) -> Self {
        let pnl = (exit_price - entry_price) * position_size;
        let pnl_pct = (exit_price - entry_price) / entry_price;
        let is_win = pnl > 0.0;

        Self {
            entry_timestamp,
            exit_timestamp,
            entry_price,
            exit_price,
            position_size,
            pnl,
            pnl_pct,
            duration_bars: 0, // Will be set by caller
            is_win,
        }
    }

    pub fn duration_days(&self) -> f64 {
        ((self.exit_timestamp - self.entry_timestamp) as f64) / (1000.0 * 60.0 * 60.0 * 24.0)
    }
}

/// Trade statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeStats {
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub largest_win: f64,
    pub largest_loss: f64,
    pub avg_trade: f64,
    pub expectancy: f64,
    pub longest_win_streak: usize,
    pub longest_loss_streak: usize,
}

impl TradeStats {
    pub fn from_trades(trades: &[Trade]) -> Self {
        if trades.is_empty() {
            return Self::default();
        }

        let total_trades = trades.len();
        let winning_trades = trades.iter().filter(|t| t.is_win).count();
        let losing_trades = total_trades - winning_trades;

        let win_rate = if total_trades > 0 {
            winning_trades as f64 / total_trades as f64
        } else {
            0.0
        };

        let wins: Vec<f64> = trades.iter().filter(|t| t.is_win).map(|t| t.pnl).collect();
        let losses: Vec<f64> = trades
            .iter()
            .filter(|t| !t.is_win)
            .map(|t| t.pnl.abs())
            .collect();

        let total_wins: f64 = wins.iter().sum();
        let total_losses: f64 = losses.iter().sum();

        let profit_factor = if total_losses > 0.0 {
            total_wins / total_losses
        } else if total_wins > 0.0 {
            999.0 // No losses
        } else {
            0.0
        };

        let avg_win = if !wins.is_empty() {
            total_wins / wins.len() as f64
        } else {
            0.0
        };

        let avg_loss = if !losses.is_empty() {
            total_losses / losses.len() as f64
        } else {
            0.0
        };

        let largest_win = wins.iter().fold(0.0f64, |a, &b| a.max(b));
        let largest_loss = losses.iter().fold(0.0f64, |a, &b| a.max(b));

        let total_pnl: f64 = trades.iter().map(|t| t.pnl).sum();
        let avg_trade = total_pnl / total_trades as f64;

        let expectancy = (win_rate * avg_win) - ((1.0 - win_rate) * avg_loss);

        // Calculate streaks
        let mut longest_win_streak = 0;
        let mut longest_loss_streak = 0;
        let mut current_win_streak = 0;
        let mut current_loss_streak = 0;

        for trade in trades {
            if trade.is_win {
                current_win_streak += 1;
                current_loss_streak = 0;
                longest_win_streak = longest_win_streak.max(current_win_streak);
            } else {
                current_loss_streak += 1;
                current_win_streak = 0;
                longest_loss_streak = longest_loss_streak.max(current_loss_streak);
            }
        }

        Self {
            total_trades,
            winning_trades,
            losing_trades,
            win_rate,
            profit_factor,
            avg_win,
            avg_loss,
            largest_win,
            largest_loss,
            avg_trade,
            expectancy,
            longest_win_streak,
            longest_loss_streak,
        }
    }
}

impl Default for TradeStats {
    fn default() -> Self {
        Self {
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            win_rate: 0.0,
            profit_factor: 0.0,
            avg_win: 0.0,
            avg_loss: 0.0,
            largest_win: 0.0,
            largest_loss: 0.0,
            avg_trade: 0.0,
            expectancy: 0.0,
            longest_win_streak: 0,
            longest_loss_streak: 0,
        }
    }
}
