#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

#[derive(PartialEq, Eq, Clone, Copy)]
enum UsagePage {
    GenericDesktop,
    KeyboardKeypad,
    Button,
    Consumer,
}

impl UsagePage {
    const fn value<const USAGE_PAGE: UsagePage>() -> u8 {
        match USAGE_PAGE {
            GenericDesktop => 0x01,
            KeyboardKeypad => 0x07,
            Button => 0x09,
            Consumer => 0x0c,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Usage {
    Keyboard,
    Mouse,
    Pointer,
    X,
    Y,
    Wheel,
    AcPan,
}

impl Usage {
    const fn size(self) -> usize {
        match self {
            AcPan => 2,
            _ => 1,
        }
    }
    const fn value<const USAGE: Usage>() -> [u8; USAGE.size()] {
        let mut rv = [0; USAGE.size()];
        match USAGE {
            Keyboard => {
                rv[0] = 0x06;
            }
            Mouse => {
                rv[0] = 0x02;
            }
            Pointer => {
                rv[0] = 0x01;
            }
            X => {
                rv[0] = 0x30;
            }
            Y => {
                rv[0] = 0x31;
            }
            Wheel => {
                rv[0] = 0x38;
            }
            AcPan => {
                rv[0] = 0x38;
                rv[1] = 0x02;
            }
        }
        return rv;
    }
}

const fn extend<const N: usize, const M: usize>(a: [u8; N]) -> [u8; M] {
    let mut new = [0; M];
    let mut i = 0;
    while i < N {
        new[i] = a[i];
        i += 1;
    }
    new
}

struct HIDBuilder<const N: usize> {
    hid_bytes: [u8; N],
}

impl HIDBuilder<0> {
    const fn new() -> HIDBuilder<0> {
        HIDBuilder { hid_bytes: [] }
    }
}

impl<const N: usize> HIDBuilder<N> {
    const fn usage_page<const USAGE_PAGE: UsagePage>(self) -> HIDBuilder<{ N + 2 }> {
        let mut new_bytes = extend::<{ N }, { N + 2 }>(self.hid_bytes);
        new_bytes[N] = 0x05;
        new_bytes[N + 1] = UsagePage::value::<USAGE_PAGE>();
        HIDBuilder {
            hid_bytes: new_bytes,
        }
    }
    const fn usage<const USAGE: Usage>(self) -> HIDBuilder<{ N + 1 + USAGE.size() }> {
        let mut new_bytes = extend::<{ N }, { N + 1 + USAGE.size() }>(self.hid_bytes);
        if USAGE.size() == 2 {
            new_bytes[N] = 0x0a;
        } else {
            new_bytes[N] = 0x09;
        }
        let v = Usage::value::<USAGE>();
        match v.len() {
            1 => {
                new_bytes[N + 1] = v[0];
            }
            2 => {
                new_bytes[N + 1] = v[0];
                new_bytes[N + 2] = v[1];
            }
            _ => {
                unreachable!()
            }
        }
        HIDBuilder {
            hid_bytes: new_bytes,
        }
    }
    const fn padding<const P: u8>(self) -> HIDBuilder<{ N + 4 }> {
        let mut new_bytes = extend::<{ N }, { N + 4 }>(self.hid_bytes);
        new_bytes[N] = 0x75; // report size
        new_bytes[N + 1] = 0x1; // report size 1
        new_bytes[N + 2] = 0x95; // report count
        new_bytes[N + 3] = P; // report count P

        HIDBuilder {
            hid_bytes: new_bytes,
        }
    }
    const fn as_bytes(self) -> [u8; N] {
        self.hid_bytes
    }
}

const hid: [u8; 9] = HIDBuilder::new()
    .usage_page::<{ UsagePage::GenericDesktop }>()
    .usage::<{ Usage::AcPan }>()
    .padding::<3>()
    .as_bytes();

fn main() {
    println!("{:?}", hid);
}
