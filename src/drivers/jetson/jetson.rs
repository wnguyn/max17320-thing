// TODO: make this also work for rx? but like documentation
// only says the bms connects to rx thourgh? may need to recieve data later but
use stm32h7xx_hal::{block, hal::serial::Write};
use usb_device::bus::*;
use usb_device::{
    bus::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::SerialPort;

use usbd_serial::USB_CLASS_CDC;
// change uart later??
pub struct JetsonUART<T> {
    tx: T,
}

impl<T: Write<u8>> JetsonUART<T> {
    pub fn new(tx: T) -> Self {
        Self { tx }
    }

    pub fn send_byte(&mut self, byte: u8) -> Result<(), T::Error> {
        block!(self.tx.write(byte))
    }

    pub fn send_bytes(&mut self, bytes: &[u8]) -> Result<(), T::Error> {
        for &byte in bytes {
            self.send_byte(byte)?;
        }
        block!(self.tx.flush())
    }
    pub fn recv(&mut self) -> Result<Option<u8>, E> {
        match self.uart.read() {
            Ok(byte) => Ok(Some(byte)),

            Err(nb::Error::WouldBlock) => Ok(None),

            Err(nb::Error::Other(e)) => Err(e),
        }
    }
}

// ideally the usb bus should life for lifetime anyways.
// i don't know doing generics is more fun. this is why i didn't do this in C.

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
    // byte impementation later??? i don't know how usb works tbh LOL
    pub fn send(&mut self, bytes: &[u8]) -> Result<usize, usb_device::UsbError> {
        self.serial.write(bytes)
    }
}
