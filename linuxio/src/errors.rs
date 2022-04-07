use thiserror::Error;

pub type USBResult = Result<(), USBError>;

#[derive(Debug, Error)]
pub enum USBError {
    #[error("IO error")]
    IOError(#[from] std::io::Error),

    #[error("Usb device error")]
    DeviceError(#[from] libusb::Error),
}