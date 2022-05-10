use std::fmt;

pub struct Ip {
    address: String,
}

impl Ip {
    pub fn get(self) -> String {
        self.address
    }

    pub fn new(addr: String) -> Ip {
        Ip { address: addr }
    }
}

impl fmt::Display for Ip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Address: {}", self.address)
    }
}

pub enum IpAddr {
    V4(Ip),
    V6(Ip),
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V4(Ip) => write!(f, "V4 {}", Ip),
            Self::V6(Ip) => write!(f, "V6 {}", Ip),
        }
    }
}

#[derive(Debug)]
pub enum Currency {
    USD,
    VND,
}

pub enum Coin {
    BTC(Currency),
    ETH(Currency),
    DOT(Currency),
}

impl Coin {
    pub fn get_ticket(coin: Coin) -> u64 {
        match coin {
            Coin::BTC(currency) => match currency {
                Currency::USD => 50000,
                Currency::VND => 50000 * 23000,
            },
            Coin::ETH(currency) => match currency {
                Currency::USD => 3000,
                Currency::VND => 3000 * 23000,
            },
            Coin::DOT(currency) => match currency {
                Currency::USD => 100,
                Currency::VND => 100 * 23000,
            },
        }
    }
}
