use super::*;
use std::{array, io};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address<const N: usize = 1>(pub Ip, pub [Port; N]);

impl<const N: usize> Address<N> {
    #[inline]
    pub fn first(&self) -> SocketAddr {
        SocketAddr::new(self.0, self.1[0].into())
    }
    #[inline]
    pub fn last(&self) -> SocketAddr {
        SocketAddr::new(self.0, self.1[N - 1].into())
    }
    #[inline]
    pub fn iter(&self) -> array::IntoIter<SocketAddr, N> {
        self.into_iter()
    }
}

impl<const N: usize> IntoIterator for Address<N> {
    type Item = SocketAddr;
    type IntoIter = array::IntoIter<SocketAddr, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.1
            .map(move |port| SocketAddr::new(self.0, port.0))
            .into_iter()
    }
}

impl<const N: usize> ToSocketAddrs for Address<N> {
    type Iter = array::IntoIter<SocketAddr, N>;
    #[inline]
    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        Ok(self.into_iter())
    }
}

pub trait Dock {
    fn to_ip(&self) -> Ip;
    #[inline]
    fn with<const N: usize>(&self, ports: [impl Into<Port>; N]) -> Address<N> {
        let ports = ports.map(|port| port.into());
        Address(self.to_ip(), ports)
    }
}

impl Dock for Ip {
    #[inline]
    fn to_ip(&self) -> Ip {
        *self
    }
}
