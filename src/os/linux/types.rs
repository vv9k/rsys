use super::Error;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MountPoint {
    volume: String,
    path: String,
    voltype: String,
    options: String,
}
impl MountPoint {
    fn new(volume: &str, path: &str, voltype: &str, options: &str) -> MountPoint {
        MountPoint {
            volume: volume.to_string(),
            path: path.to_string(),
            voltype: voltype.to_string(),
            options: options.to_string(),
        }
    }
    pub(crate) fn from_line(line: &str) -> Option<MountPoint> {
        let mut elems = line.split_ascii_whitespace().take(4);
        if elems.clone().count() >= 4 {
            let volume = elems.next().unwrap();
            let path = elems.next().unwrap();
            let voltype = elems.next().unwrap();
            let options = elems.next().unwrap();
            return Some(Self::new(volume, path, voltype, options));
        }

        None
    }
}

pub type MountPoints = Vec<MountPoint>;
pub type Ifaces = Vec<IfaceDev>;

macro_rules! next_u64 {
    ($list:ident) => {
        $list
            .next()
            .unwrap()
            .parse::<u64>()
            .map_err(|e| Error::InvalidInputError(e.to_string()))?
    };
}

#[derive(Debug, Default, Eq, PartialEq)]
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

                rx_bytes: next_u64!(elems),
                rx_packets: next_u64!(elems),
                rx_errs: next_u64!(elems),
                rx_drop: next_u64!(elems),
                rx_fifo: next_u64!(elems),
                rx_frame: next_u64!(elems),
                rx_compressed: next_u64!(elems),
                rx_multicast: next_u64!(elems),

                tx_bytes: next_u64!(elems),
                tx_packets: next_u64!(elems),
                tx_errs: next_u64!(elems),
                tx_drop: next_u64!(elems),
                tx_fifo: next_u64!(elems),
                tx_frame: next_u64!(elems),
                tx_compressed: next_u64!(elems),
                tx_multicast: next_u64!(elems),
            });
        }

        Err(Error::InvalidInputError(
            "Line contains invalid proc/net/dev output".to_string(),
        ))
    }
}
