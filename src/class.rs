// use super::buffer::AudioBuffer;
use usb_device::class_prelude::*;
use usb_device::Result;

// const FEATURE_UNIT_ID: u8 = 0x16;
const INPUT_TERMINAL_ID: u8 = 0x01;
const OUTPUT_TERMINAL_ID: u8 = 0x02;

const CHANNEL_COUNT: u32 = 2;
const SAMPLE_RESOLUTION: u32 = 16;
const SAMPLING_FREQUENCY: u32 = 48000;
const MAX_PACKET_SIZE: u32 = (SAMPLING_FREQUENCY * CHANNEL_COUNT * 2) / 1000;
// const BUFFER_SIZE: usize = (MAX_PACKET_SIZE * 500) as usize;

// const MIN_VOLUME: i16 = -25 * 256; // -25db * 256 steps, as per UAC
// const MAX_VOLUME: i16 = 6 * 256;
// const DEFAULT_VOLUME: i16 = 0;

// enum RequestCode {
//     Undefined = 0x0,
//     SetCur = 0x01,
//     GetCur = 0x81,
//     SetMin = 0x02,
//     GetMin = 0x82,
//     SetMax = 0x03,
//     GetMax = 0x83,
//     SetRes = 0x04,
//     GetRes = 0x84,
//     SetMem = 0x05,
//     GetMem = 0x85,
//     GetStat = 0xFF,
// }

pub struct AudioClass<'a, B: UsbBus> {
    // pub struct AudioClass<B: UsbBus> {
    control_if: InterfaceNumber,
    play_data_if: InterfaceNumber,
    play_data_ep: EndpointOut<'a, B>, // feature_unit: FeatureUnit,
    // audio_buffer: AudioBuffer,
    // sample_buffer: [u8; BUFFER_SIZE],
    // write_pos: usize,
}

// struct FeatureUnit {
//     muted: bool,
//     current_volume: i16,
// }

// impl FeatureUnit {
//     fn new(muted: bool, default_volume: i16) -> FeatureUnit {
//         FeatureUnit {
//             muted,
//             current_volume: default_volume,
//         }
//     }
//

impl<B: UsbBus> AudioClass<'_, B> {
    pub fn new(alloc: &UsbBusAllocator<B>) -> AudioClass<B> {
        AudioClass {
            control_if: alloc.interface(),
            play_data_if: alloc.interface(),
            play_data_ep: alloc.isochronous(MAX_PACKET_SIZE as u16, true), // feature_unit: FeatureUnit::new(false, DEFAULT_VOLUME),
            // audio_buffer: AudioBuffer::new(),
            // sample_buffer: [0; BUFFER_SIZE],
            // write_pos: 0,
        }
    }

    pub fn read_data(&self, buf: &mut [u8]) -> Result<usize> {
        self.play_data_ep.read(buf)
    }

    // pub fn audio_buffer(&mut self) -> &mut AudioBuffer {
    //     &mut self.audio_buffer
    // }

    //     fn feature_unit_handle_out(&mut self, interface: u8, xfer: ControlOut<B>) {
    //         if interface != 0 || interface != u8::from(self.play_data_if) {
    //             xfer.reject().ok();
    //             return;
    //         }

    //         let (control, channel) = (
    //             ((xfer.request().value >> 8) & 0xFF) as u8,
    //             (xfer.request().value & 0xFF) as u8,
    //         );

    //         // logical channels are 1-indexed, so master channel should be 1
    //         if (control != 1 || control != 2) && (channel != 0xFF || channel != 1) {
    //             xfer.reject().ok();
    //             return;
    //         }

    //         let request: RequestCode = unsafe { mem::transmute(xfer.request().request) };

    //         match (control, request) {
    //             (1, RequestCode::SetCur) => {
    //                 self.feature_unit.muted = if xfer.data()[0] & 1 == 1 { true } else { false };
    //                 xfer.accept().ok();
    //             }
    //             (2, RequestCode::SetCur) => {
    //                 self.feature_unit.current_volume =
    //                     i16::from_le_bytes(xfer.data()[0..2].try_into().unwrap());
    //                 xfer.accept().ok();
    //             }
    //             (_, _) => {
    //                 xfer.reject().ok();
    //             }
    //         };
    //     }

    //     fn feature_unit_handle_in(&mut self, interface: u8, xfer: ControlIn<B>) {
    //         if interface != 0 || interface != u8::from(self.play_data_if) {
    //             xfer.reject().ok();
    //             return;
    //         }

    //         let (control, channel) = (
    //             ((xfer.request().value >> 8) & 0xFF) as u8,
    //             (xfer.request().value & 0xFF) as u8,
    //         );

    //         if (control != 1 || control != 2) && (channel != 0xFF || channel != 1) {
    //             xfer.reject().ok();
    //             return;
    //         }

    //         let request: RequestCode = unsafe { mem::transmute(xfer.request().request) };

    //         match (control, request) {
    //             (1, RequestCode::GetCur) => {
    //                 xfer.accept_with(&[if self.feature_unit.muted { 1u8 } else { 0u8 }])
    //                     .ok();
    //             }
    //             (2, RequestCode::GetMax) => {
    //                 xfer.accept_with_static(&[
    //                     (MAX_VOLUME & 0xFF) as u8,
    //                     ((MAX_VOLUME >> 8) & 0xFF) as u8,
    //                 ])
    //                 .ok();
    //             }
    //             (2, RequestCode::GetMin) => {
    //                 xfer.accept_with_static(&[
    //                     (MIN_VOLUME & 0xFF) as u8,
    //                     ((MIN_VOLUME >> 8) & 0xFF) as u8,
    //                 ])
    //                 .ok();
    //             }
    //             (2, RequestCode::GetCur) => {
    //                 xfer.accept_with(&[
    //                     (self.feature_unit.current_volume & 0xFF) as u8,
    //                     ((self.feature_unit.current_volume >> 8) & 0xFF) as u8,
    //                 ])
    //                 .ok();
    //             }
    //             (_, _) => {
    //                 xfer.reject().ok();
    //             }
    //         }
    //     }
}

impl<B> UsbClass<B> for AudioClass<'_, B>
where
    B: UsbBus,
{
    /// Called when a GET_DESCRIPTOR request is received for a configuration descriptor. When
    /// called, the implementation should write its interface, endpoint and any extra class
    /// descriptors into `writer`. The configuration descriptor itself will be written by
    /// [UsbDevice](crate::device::UsbDevice) and shouldn't be written by classes.
    ///
    /// # Errors
    ///
    /// Generally errors returned by `DescriptorWriter`. Implementors should propagate any errors
    /// using `?`.
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        // Standard AC Interface Descriptor: Audio control interface
        writer.interface(self.control_if, 1, 1, 0, 0)?;

        // USB OUT Terminal for play session
        // Input Terminal Descriptor
        let input_terminal = &[
            0x02,                  // bDescriptorSubtype - Input terminal
            INPUT_TERMINAL_ID, // bTerminalID (must be unique, higher than highest interface number to avoid confusion)
            (0x0101 & 0xFF) as u8, // wTerminalType - USB streaming
            ((0x0101 >> 8) & 0xFF) as u8,
            0x00, // bAssocTerminal - ID of the output terminal associated to this input terminal
            CHANNEL_COUNT as u8, // bNrChannels - stereo
            (0x03 & 0xFF) as u8, // wChannelConfig - channel configuration map (stereo)
            ((0x03 >> 8) & 0xFF) as u8,
            0x00, // iChannelNames
            0x00, // iTerminal
        ];

        // USB Play control feature
        // Feature Unit Descriptor
        // let feature_unit = &[
        //     0x06,            // bDescriptorSubtype - Feature unit
        //     FEATURE_UNIT_ID, // bUnitID - unique identifier
        //     0x12,            // bSourceID - ID of terminal associated with this feature unit
        //     0x01, // bControlSize - bmaControls member size in bytes (size of one bmaControls map)
        //     0x01 | 0x02, // bmaControls(0) - volume and mute (on master channel)
        //     0x00, // bmaControls(1) - logical channel 1 features (volume for left and right channel is doable)
        //     0x00, // bmaControls(2) - logical channel 2 features
        //     0x00, // iFeature
        // ];

        // USB Play : Speaker Terminal
        // Output Terminal Descriptor
        let output_terminal = &[
            0x03,                  // bDescriptorSubtype - Output terminal
            OUTPUT_TERMINAL_ID,    // bTerminalID - unique identifier
            (0x0301 & 0xFF) as u8, // wTerminalType - speaker
            ((0x0301 >> 8) & 0xFF) as u8,
            0x00,              // bAssocTerminal
            INPUT_TERMINAL_ID, // input terminal //FEATURE_UNIT_ID, // bSourceID - associated Feature Unit ID
            0x00,              // iTerminal
        ];

        // USB play Standard AS Interface Descriptor - Audio Streaming Zero Bandwith
        // Standard AS Interface Descriptor
        // let standard_as_zero = &[
        //     u8::from(self.play_data_if), // bInterfaceNumber
        //     0x00,                        // bAlternateSetting
        //     0x00,                        // bNumEndpoints
        //     0x01,                        // bInterfaceClass
        //     0x02,                        // bInterfaceSubClass
        //     0x00,                        // bInterfaceProtocol
        //     0x00,                        // iInterface
        // ];

        // USB play Standard AS Interface Descriptors - Audio streaming operational
        // Standard AS Interface Descriptor
        // let standard_as_operational = &[
        //     u8::from(self.play_data_if), // bInterfaceNumber
        //     0x01,                        // bAlternateSetting
        //     0x01,                        // bNumEndpoints
        //     0x01,                        // bInterfaceClass
        //     0x02,                        // bInterfaceSubClass
        //     0x00,                        // bInterfaceProtocol
        //     0x00,                        // iInterface
        // ];

        // Class-Specific AS Interface Descriptor
        let as_interface = &[
            0x01,                  // bDescriptorSubtype - Class-Specific AudioStreaming interface descriptor
            INPUT_TERMINAL_ID,     // bTerminalLink - ID of terminal associated with this interface
            0x01,                  // bDelay - see spec (I've got no clue)
            (0x0001 & 0xFF) as u8, // wFormatTag - PCM
            ((0x0001 >> 8) & 0xFF) as u8,
        ];

        // Audio Type I Format descriptor
        let format = &[
            0x02, // bDescriptorSubtype - Format type I (describes the PCM format selected above with wFormatTag)
            0x01, // bFormatType - Format type I
            0x02, // bNrChannels - 2 (stereo)
            ((SAMPLE_RESOLUTION + 7) / 8) as u8, // bSubframeSize - 2 bytes per sample (16-bit resolution)
            SAMPLE_RESOLUTION as u8,             // bBitResolution
            1, // bSamFreqType - number of different sampling frequencies supported (1 - 48kHz)
            (SAMPLING_FREQUENCY & 0xFF) as u8, // tSamFreq[1]
            ((SAMPLING_FREQUENCY >> 8) & 0xFF) as u8,
            ((SAMPLING_FREQUENCY >> 16) & 0xFF) as u8,
        ];

        // USB Play data ep
        // Standard AS Isochronous Audio Data Endpoint Descriptor
        // Although being standard descriptor, it's actually specific to UAC as it has two more fields (bRefresh, bSynchAddress)
        // let isochronous_data_ep = &[
        //     self.play_data_ep.address().index() as u8, // bEndpointAddress - OUT EP 1
        //     0x01,                                      // bmAttributes - Isochronous
        //     (MAX_PACKET_SIZE & 0xFF) as u8, // wMaxPacketSize  // Freq(Samples)*2(Stereo)*2(HalfWord)
        //     ((MAX_PACKET_SIZE >> 8) & 0xFF) as u8,
        //     0x01, // bInterval - must be set to 1
        //     0x00, // bRefresh - must be reset to 0
        //     0x00, // bSynchAddress - no synchronization endpoint used at the moment
        // ];

        // Class-Specific AS Isochronous Audio Data Endpoint Descriptor
        let class_specific_iso_data_ep = &[
            0x01, // bDescriptorSubtype - general
            0x00, // bmAttributes - here you can allow sampling frequency changing
            0x00, // bLockDelayUnits
            0x00, // wLockDelay
            0x00, // wLockDelay
        ];

        let length = 9
            + input_terminal.len()
            + 2
            // + feature_unit.len()
            // + 2
            + output_terminal.len()
            + 2;

        // info!("Length = {}", length);

        // Class-Specific AC Interface Header Descriptor
        let interface_header = &[
            0x01,                  // bDescriptorSubtype
            (0x0100 & 0xFF) as u8, // bcdADC - Audio device class release number (BCD coded)
            ((0x0100 >> 8) & 0xFF) as u8,
            (length & 0xFF) as u8, // wTotalLength - includes units descriptors length (2 bytes)
            ((length >> 8) & 0xFF) as u8,
            0x01,                        // bInCollection - Number of audio streaming interfaces
            u8::from(self.play_data_if), // baInterfaceNr - First streaming interface number
        ];

        writer.write(0x24, interface_header)?;
        writer.write(0x24, input_terminal)?;
        // writer.write(0x24, feature_unit)?;
        writer.write(0x24, output_terminal)?;
        // writer.write(0x04, standard_as_zero)?;
        // writer.write(0x04, standard_as_operational)?;
        writer.interface(self.play_data_if, 0x01, 0x02, 0x00, 0)?;
        writer.interface(self.play_data_if, 0x01, 0x02, 0x00, 1)?;
        writer.write(0x24, as_interface)?;
        writer.write(0x24, format)?;
        writer.endpoint(&self.play_data_ep)?;
        // writer.write(0x05, isochronous_data_ep)?;
        writer.write(0x25, class_specific_iso_data_ep)?;

        // info!("Descriptors written");

        Ok(())
    }

    /// Gets a class-specific string descriptor.
    ///
    /// Note: All string descriptor requests are passed to all classes in turn, so implementations
    /// should return [`None`] if an unknown index is requested.
    ///
    /// # Arguments
    ///
    /// * `index` - A string index allocated earlier with
    ///   [`UsbAllocator`](crate::bus::UsbBusAllocator).
    /// * `lang_id` - The language ID for the string to retrieve.
    fn get_string(&self, index: StringIndex, lang_id: u16) -> Option<&str> {
        let _ = (index, lang_id);
        None
    }

    /// Called after a USB reset after the bus reset sequence is complete.
    fn reset(&mut self) {
        // info!("Reset complete");
    }

    //    fn configure(&mut self) {}

    /// Called whenever the `UsbDevice` is polled.
    fn poll(&mut self) {
        // info!("Poll");

        // let mut buffer: [u8; MAX_PACKET_SIZE as usize] = [0; MAX_PACKET_SIZE as usize];

        // match self.play_data_ep.read(&mut buffer) {
        //     Ok(size) => {
        //         for bytes in buffer.chunks_exact(2) {
        //             let sample = ((bytes[0] as u16) << 8) | bytes[1] as u16;

        //             unsafe {
        //                 let i2s = stm32f4xx_hal::stm32::Peripherals::steal().SPI2;

        //                 while i2s.sr.read().txe().bit_is_clear() {}

        //                 i2s.dr.write(|w| unsafe { w.dr().bits(sample) });
        //             }
        //         }
        //     }
        //     Err(UsbError::WouldBlock) => (),
        //     Err(_) => info!("playback data read error"),
        // }
    }

    /// Called when a control request is received with direction HostToDevice.
    ///
    /// All requests are passed to classes in turn, which can choose to accept, ignore or report an
    /// error. Classes can even choose to override standard requests, but doing that is rarely
    /// necessary.
    ///
    /// See [`ControlOut`] for how to respond to the transfer.
    ///
    /// When implementing your own class, you should ignore any requests that are not meant for your
    /// class so that any other classes in the composite device can process them.
    ///
    /// # Arguments
    ///
    /// * `req` - The request from the SETUP packet.
    /// * `xfer` - A handle to the transfer.
    fn control_out(&mut self, xfer: ControlOut<B>) {
        let req = xfer.request();

        if req.request_type == control::RequestType::Standard
            && req.recipient == control::Recipient::Interface
        {
            // TODO handle alternate settings
        }

        if !(req.request_type == control::RequestType::Class
            && req.recipient == control::Recipient::Interface)
        {
            return;
        }

        // let (entity_id, interface_id) = (((req.index >> 8) & 0xFF) as u8, (req.index & 0xFF) as u8);

        // match (entity_id, interface_id) {
        //     (FEATURE_UNIT_ID, _) => {
        //         self.feature_unit_handle_out(interface_id, xfer);
        //     }
        //     (_, _) => {
        //         xfer.reject().ok();
        //     }
        // };

        // info!(
        //     "Control OUT - req: {}, type: {}, recipient: {}, length: {}, index: {}, value: {}",
        //     req.request,
        //     req.request_type as u8,
        //     req.recipient as u8,
        //     req.length,
        //     req.index,
        //     req.value
        // );
    }

    /// Called when a control request is received with direction DeviceToHost.
    ///
    /// All requests are passed to classes in turn, which can choose to accept, ignore or report an
    /// error. Classes can even choose to override standard requests, but doing that is rarely
    /// necessary.
    ///
    /// See [`ControlIn`] for how to respond to the transfer.
    ///
    /// When implementing your own class, you should ignore any requests that are not meant for your
    /// class so that any other classes in the composite device can process them.
    ///
    /// # Arguments
    ///
    /// * `req` - The request from the SETUP packet.
    /// * `data` - Data to send in the DATA stage of the control transfer.
    fn control_in(&mut self, xfer: ControlIn<B>) {
        let req = xfer.request();

        if !(req.request_type == control::RequestType::Class
            && req.recipient == control::Recipient::Interface)
        {
            return;
        }

        // let (entity_id, interface_id) = (((req.index >> 8) & 0xFF) as u8, (req.index & 0xFF) as u8);

        // match (entity_id, interface_id) {
        //     (FEATURE_UNIT_ID, _) => {
        //         self.feature_unit_handle_in(interface_id, xfer);
        //     }
        //     (_, _) => {
        //         xfer.reject().ok();
        //     }
        // };

        // info!(
        //     "Control IN - req: {}, type: {}, recipient: {}, length: {}, index: {}, value: {}",
        //     req.request,
        //     req.request_type as u8,
        //     req.recipient as u8,
        //     req.length,
        //     req.index,
        //     req.value
        // );
    }

    /// Called when endpoint with address `addr` has received a SETUP packet. Implementing this
    /// shouldn't be necessary in most cases, but is provided for completeness' sake.
    ///
    /// Note: This method may be called for an endpoint address you didn't allocate, and in that
    /// case you should ignore the event.
    fn endpoint_setup(&mut self, addr: EndpointAddress) {
        //    info!("SETUP packet received on {}", addr.index());
    }

    /// Called when endpoint with address `addr` has received data (OUT packet).
    ///
    /// Note: This method may be called for an endpoint address you didn't allocate, and in that
    /// case you should ignore the event.
    fn endpoint_out(&mut self, addr: EndpointAddress) {
        //    info!("Packet received on {}", addr.index());
    }

    /// Called when endpoint with address `addr` has completed transmitting data (IN packet).
    ///
    /// Note: This method may be called for an endpoint address you didn't allocate, and in that
    /// case you should ignore the event.
    fn endpoint_in_complete(&mut self, addr: EndpointAddress) {
        //    info!("Transfer complete on {}", addr.index());
    }
}
