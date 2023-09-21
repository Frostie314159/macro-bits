// This example shows how to parse the frame control field of a IEEE802.11 frame.
use macro_bits::{bit, bitfield, serializable_enum};

serializable_enum! {
    #[derive(Debug, PartialEq)]
    pub enum Type: u8 {
        Management => 0x00,
        Control => 0x01,
        Data => 0x02,
        Extension => 0x03
    }
}
serializable_enum! {
    #[derive(Debug, PartialEq)]
    pub enum ManagementSubType: u8 {
        ProbeRequest => 0x04
    }
}
bitfield! {
    #[derive(Debug, PartialEq, Default)]
    pub struct FCFFlags: u8 {
        pub to_ds: bool => bit!(0),
        pub from_ds: bool => bit!(1),
        pub more_fragments: bool => bit!(2),
        pub retry: bool => bit!(3),
        pub pwr_mgt: bool => bit!(4),
        pub more_data: bool => bit!(5),
        pub protected_flags: bool => bit!(6),
        pub htc_plus: bool => bit!(7)
    }
}
bitfield! {
    #[derive(Debug, PartialEq)]
    pub struct FrameControlField: u16 {
        pub version: u8 => bit!(0,1),
        pub frame_type: Type => bit!(2,3),
        pub frame_sub_type: ManagementSubType => bit!(4,5,6,7),
        pub flags: FCFFlags => 0xff00
    }
}
fn main() {
    let raw = 0x40;
    let target = FrameControlField {
        version: 0,
        frame_type: Type::Management,
        frame_sub_type: ManagementSubType::ProbeRequest,
        flags: FCFFlags::default()
    };
    assert_eq!(target, FrameControlField::from_representation(raw));
    assert_eq!(target.to_representation(), raw);
}
