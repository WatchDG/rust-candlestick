#[derive(Debug, Clone)]
pub struct Candlestick<Instrument, Interval, Time, Price, Volume>
where
    Instrument: Clone,
    Interval: Clone,
    Time: Clone,
    Price: Clone,
    Volume: Clone,
{
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
where
    Instrument: Clone,
    Interval: Clone,
    Time: Clone,
    Price: Clone,
    Volume: Clone,
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

#[cfg(feature = "time_series")]
extern crate time_series;

#[cfg(feature = "time_series")]
use time_series::DataPoint;

#[cfg(feature = "time_series")]
impl<Instrument, Interval, Time, Price, Volume>
    Candlestick<Instrument, Interval, Time, Price, Volume>
where
    Instrument: Clone,
    Interval: Clone,
    Time: Clone,
    Price: Clone,
    Volume: Clone,
{
    pub fn open_to_datapoint(&self) -> DataPoint<Time, Price> {
        DataPoint {
            time: self.time.clone(),
            data: self.open.clone(),
        }
    }

    pub fn high_to_datapoint(&self) -> DataPoint<Time, Price> {
        DataPoint {
            time: self.time.clone(),
            data: self.high.clone(),
        }
    }

    pub fn low_to_datapoint(&self) -> DataPoint<Time, Price> {
        DataPoint {
            time: self.time.clone(),
            data: self.low.clone(),
        }
    }

    pub fn close_to_datapoint(&self) -> DataPoint<Time, Price> {
        DataPoint {
            time: self.time.clone(),
            data: self.close.clone(),
        }
    }

    pub fn volume_to_datapoint(&self) -> DataPoint<Time, Volume> {
        DataPoint {
            time: self.time.clone(),
            data: self.volume.clone(),
        }
    }
}
