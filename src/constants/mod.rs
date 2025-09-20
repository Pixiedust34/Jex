use bitflags::bitflags;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrudpPacketType {
	Syn = 0x0,
	Connect = 0x1,
	Data = 0x2,
	Disconnect = 0x3,
	Ping = 0x4,
}

bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct PrudpPacketFlags: u16 {
		const ACK        = 0x0001;
		const RELIABLE   = 0x0002;
		const NEEDS_ACK  = 0x0004;
		const HAS_SIZE   = 0x0008;
		const MULTI_ACK  = 0x0200;
	}
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureMethod {
	Method0 = 0,
	ConnectionAddress = 1,
	Method2 = 2,
	Method3 = 3,
	Method4 = 4,
	Method5 = 5,
	UseKey = 6,
	Method7 = 7,
	UseEntropy = 8,
	Ignore = 9,
}

bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct StationUrlFlag: u8 {
		const BEHIND_NAT = 0x01;
		const PUBLIC     = 0x02;
	}
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StationUrlType {
	Unknown = 0,
	Prudp = 1,
	Prudps = 2,
	Udp = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NatFilteringProperties {
	Unknown = 0,
	PortIndependent = 1,
	PortDependent = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NatMappingProperties {
	Unknown = 0,
	EndpointIndependent = 1,
	EndpointDependent = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
	Do = 1,
	Rv = 2,
	OldRvSec = 3,
	SbMgmt = 4,
	Nat = 5,
	SessionDiscovery = 6,
	NatEcho = 7,
	Routing = 8,
	Game = 9,
	RvSecure = 10,
	Relay = 11,
}