#[derive(Debug)]
pub struct Candlestick<Instrument, Interval, Time, Price, Volume> {
    pub instrument: Instrument,
    pub interval: Interval,
    pub time: Time,
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
    pub volume: Volume,
}

impl<Instrument, Interval, Time, Price, Volume>
    Candlestick<Instrument, Interval, Time, Price, Volume>
{
    pub fn new(
        instrument: Instrument,
        interval: Interval,
        time: Time,
        open: Price,
        high: Price,
        low: Price,
        close: Price,
        volume: Volume,
    ) -> Candlestick<Instrument, Interval, Time, Price, Volume> {
        Candlestick {
            instrument,
            interval,
            time,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}
