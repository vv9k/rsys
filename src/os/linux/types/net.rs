#[cfg(test)]
use super::mocks::NET_DEV;
use super::Error;

pub type Ifaces = Vec<IfaceDev>;

macro_rules! next_u64 {
    ($list:ident, $line:ident) => {
        $list
            .next()
            .unwrap()
            .parse::<u64>()
            .map_err(|e| Error::InvalidInputError($line.to_string(), e.to_string()))?
    };
}

#[derive(Debug, Default, Eq, PartialEq)]
/// Represents a data line of /proc/net/dev
pub struct IfaceDev {
    pub iface: String,

    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_errs: u64,
    pub rx_drop: u64,
    pub rx_fifo: u64,
    pub rx_frame: u64,
    pub rx_compressed: u64,
    pub rx_multicast: u64,

    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_errs: u64,
    pub tx_drop: u64,
    pub tx_fifo: u64,
    pub tx_frame: u64,
    pub tx_compressed: u64,
    pub tx_multicast: u64,
}
impl IfaceDev {
    pub(crate) fn from_line(line: &str) -> Result<IfaceDev, Error> {
        let mut elems = line.split_ascii_whitespace().take(17);
        if elems.clone().count() >= 17 {
            return Ok(IfaceDev {
                iface: elems.next().unwrap().trim_end_matches(':').to_string(),

                rx_bytes: next_u64!(elems, line),
                rx_packets: next_u64!(elems, line),
                rx_errs: next_u64!(elems, line),
                rx_drop: next_u64!(elems, line),
                rx_fifo: next_u64!(elems, line),
                rx_frame: next_u64!(elems, line),
                rx_compressed: next_u64!(elems, line),
                rx_multicast: next_u64!(elems, line),

                tx_bytes: next_u64!(elems, line),
                tx_packets: next_u64!(elems, line),
                tx_errs: next_u64!(elems, line),
                tx_drop: next_u64!(elems, line),
                tx_fifo: next_u64!(elems, line),
                tx_frame: next_u64!(elems, line),
                tx_compressed: next_u64!(elems, line),
                tx_multicast: next_u64!(elems, line),
            });
        }

        Err(Error::InvalidInputError(
            line.to_string(),
            "contains invalid input for IfaceDev".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_iface_from_net_dev_line() {
        let lo = IfaceDev {
            iface: "lo".to_string(),
            rx_bytes: 17776656,
            rx_packets: 127989,
            rx_errs: 0,
            rx_drop: 0,
            rx_fifo: 0,
            rx_frame: 0,
            rx_compressed: 0,
            rx_multicast: 0,

            tx_bytes: 17776656,
            tx_packets: 127989,
            tx_errs: 0,
            tx_drop: 0,
            tx_fifo: 0,
            tx_frame: 0,
            tx_compressed: 0,
            tx_multicast: 0,
        };
        let enp = IfaceDev {
            iface: "enp8s0".to_string(),
            rx_bytes: 482459368,
            rx_packets: 349468,
            rx_errs: 0,
            rx_drop: 0,
            rx_fifo: 0,
            rx_frame: 0,
            rx_compressed: 0,
            rx_multicast: 4785,

            tx_bytes: 16133415,
            tx_packets: 198549,
            tx_errs: 0,
            tx_drop: 0,
            tx_fifo: 0,
            tx_frame: 0,
            tx_compressed: 0,
            tx_multicast: 0,
        };
        let mut lines = NET_DEV.split('\n').skip(2);

        assert_eq!(Ok(lo), IfaceDev::from_line(lines.next().unwrap()));
        assert_eq!(Ok(enp), IfaceDev::from_line(lines.next().unwrap()))
    }
}
