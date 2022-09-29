use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;

#[bitfield]
#[repr(u32)]
#[derive(Debug)]
pub struct Pa {
    pub a1: B1,
    pub a2: B2,
    pub a5: B5,
    pub b1: bool,
    pub b2: bool,
    pub b6: B6,
    pub s: u16
}

fn main() {
    let da = Pa::new()
    .with_a1(0b1)
    .with_a2(0b11)
    .with_a5(0b01001)
    .with_b1(true)
    .with_b2(false)
    .with_b6(0b101101)
    .with_s(0x1122);
    println!("{:?}", da);
    let bs = da.into_bytes();
    let d_u32 = u32::from_ne_bytes(bs);
    println!("{:#b}, {:#x}", d_u32, d_u32);

    let d_u16 = u16::from_ne_bytes(bs[..2].try_into().unwrap());
    println!("{:#b}, {:#x}", d_u16, d_u16);

    let dd = Pa::from(0x12345678 as u32);
    let di = u32::from(dd);
    println!("{:?}", di);
}
