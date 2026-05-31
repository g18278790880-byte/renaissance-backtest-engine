use super::*;

fn create_tick(price: i64, timestamp: u64) -> Tick {
    Tick {
        symbol: String::from("BTCUSDT"),
        price,
        quantity: 1,
        timestamp,
    }
}

mod constructor_tests {
    use super::*;

    #[test]
    fn new_creates_market_data_simulator_with_ticks() {
        let simulator =
            MarketDataSimulator::new(vec![create_tick(100_000, 1), create_tick(99_000, 2)]);

        assert_eq!(simulator.len(), 2);
        assert!(!simulator.is_empty());
    }

    #[test]
    fn demo_cross_ticks_contains_two_ticks() {
        let simulator = MarketDataSimulator::demo_cross_ticks();

        assert_eq!(simulator.len(), 2);
        assert!(!simulator.is_empty());
    }
}

mod channel_replay_tests {
    use super::*;

    #[tokio::test]
    async fn run_sends_all_ticks_to_channel() {
        let simulator =
            MarketDataSimulator::new(vec![create_tick(100_000, 1), create_tick(99_000, 2)]);

        let (tick_tx, mut tick_rx) = mpsc::channel::<Tick>(10);

        simulator.run(tick_tx).await;

        let first_tick = tick_rx.recv().await.unwrap();
        let second_tick = tick_rx.recv().await.unwrap();
        let third_tick = tick_rx.recv().await;

        assert_eq!(first_tick.price, 100_000);
        assert_eq!(first_tick.timestamp, 1);

        assert_eq!(second_tick.price, 99_000);
        assert_eq!(second_tick.timestamp, 2);

        assert!(third_tick.is_none());
    }
}

mod csv_loader_tests {
    use super::*;

    #[test]
    fn from_csv_str_loads_ticks() {
        let content = "\
timestamp,symbol,price,quantity
1,BTCUSDT,100000,1
2,BTCUSDT,99000,2
";

        let simulator = MarketDataSimulator::from_csv_str(content).unwrap();

        assert_eq!(simulator.len(), 2);
        assert!(!simulator.is_empty());
    }

    #[test]
    fn from_csv_str_rejects_invalid_header() {
        let content = "\
time,symbol,price,quantity
1,BTCUSDT,100000,1
";

        let result = MarketDataSimulator::from_csv_str(content);

        assert_eq!(result.err(), Some(MarketDataError::InvalidHeader));
    }

    #[test]
    fn from_csv_str_rejects_invalid_price() {
        let content = "\
timestamp,symbol,price,quantity
1,BTCUSDT,not_a_price,1
";

        let result = MarketDataSimulator::from_csv_str(content);

        assert_eq!(
            result.err(),
            Some(MarketDataError::InvalidPrice { line: 2 })
        );
    }
}
