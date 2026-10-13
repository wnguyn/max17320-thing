use embedded_hal_nb::serial::{Read, Write};
use usb_device::bus::*;
use usb_device::{
    bus::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

// generics to send and recieve over UART i think?
// i still don't know whether we'd use usb or UART for this?
pub struct JetsonUART<T, R> {
    tx: T,
    rx: R,
}

impl<T, R> JetsonUART<T, R>
where
    T: Write<u8>,
    R: Read<u8>,
{
    pub fn new(tx: T) -> Self {
        Self { tx }
    }

    pub fn send_byte(&mut self, byte: u8) -> Result<(), T::Error> {
        nb::block!(self.tx.write(byte))
    }

    pub fn send_bytes(&mut self, bytes: &[u8]) -> Result<(), T::Error> {
        for &byte in bytes {
            self.send_byte(byte)?;
        }
        nb::block!(self.tx.flush())
    }
    // TODO: this is super fucking janky hopefulyl this is later on issue
    pub async fn read_uart(&mut self) {
        let mut buf = [0u8; 32];
        let n = self.rx.read_until_idle(&mut buf).await.unwrap();
        let data = &buf[..n];

        // do some matching here later????
        // we have to decide what UART callss the jetson would call, but
        // i don't know dude i'm like 6-7 (SIX SEVEN!) semesters away from taking an embedded systems class
    }
}

impl<T: Read<u8>> JetsonUART<T> {
    pub fn recv(&mut self) -> Result<Option<u8>, T::Error> {
        match self.tx.read() {
            Ok(byte) => Ok(Some(byte)),

            Err(nb::Error::WouldBlock) => Ok(None),

            Err(nb::Error::Other(e)) => Err(e),
        }
    }
}
// USB interface to connect over i2c for this? but like i don't know
// we'll probably stick to basic UART communication.
pub struct JetsonUSB<T: 'static + UsbBus> {
    dev: UsbDevice<'static, T>,
    serial: SerialPort<'static, T>,
}

impl<T: 'static + UsbBus> JetsonUSB<T> {
    pub fn new(bus: &'static UsbBusAllocator<T>) -> Self {
        let serial = SerialPort::new(&bus);
        let dev = UsbDeviceBuilder::new(&bus, UsbVidPid(0x6767, 0x6767))
            .device_class(USB_CLASS_CDC)
            .build();
        Self { dev, serial }
    }
    pub fn poll(&mut self) {
        self.dev.poll(&mut [&mut self.serial]);
    }
    pub fn send(&mut self, bytes: &[u8]) -> Result<usize, usb_device::UsbError> {
        self.serial.write(bytes)
    }
}
