use embedded_hal_nb::serial::{Read, Write};
use usb_device::bus::*;
use usb_device::{
    bus::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

pub struct JetsonUART<T> {
    tx: T,
}

impl<T: Write<u8>> JetsonUART<T> {
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
