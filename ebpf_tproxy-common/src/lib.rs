#![no_std]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ipv4Addr {
    pub addr: u32,
    pub port: u32,
}

impl Ipv4Addr {
    pub fn new(addr: u32, port: u32) -> Ipv4Addr {
        Ipv4Addr { addr, port }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ipv4Tuple {
    pub protocol: u32,
    pub src: Ipv4Addr,
    pub dst: Ipv4Addr,
}

impl Ipv4Tuple {
    pub fn new(protocol: u32, src: Ipv4Addr, dst: Ipv4Addr) -> Ipv4Tuple {
        Ipv4Tuple { protocol, src, dst }
    }

    pub fn reverse(self) -> Self {
        Ipv4Tuple {
            protocol: self.protocol,
            src: self.dst,
            dst: self.src,
        }
    }
}
