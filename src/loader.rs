use std::ops::Range;
use std::sync::mpsc::{self, Receiver, Sender};

use bytes::Bytes;
use lluvia_media::containers::ts::{AdaptationFieldControl, PacketView, PACKET_SIZE};

/// Raw file payload loaded from disk or browser file picker.
pub(crate) struct LoadedFile {
    pub(crate) name: String,
    pub(crate) data: Bytes,
}

/// Asynchronous cross-platform file loader supporting both Native and Web (WASM).
pub(crate) struct FileLoader {
    tx: Sender<Result<LoadedFile, String>>,
    rx: Receiver<Result<LoadedFile, String>>,
}

impl Default for FileLoader {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }
}

impl FileLoader {
    /// Opens a native or web file dialog for selecting an MPEG Transport Stream file.
    pub(crate) fn open_file_dialog(&self, ctx: &egui::Context) {
        let tx = self.tx.clone();
        let ctx = ctx.clone();

        #[cfg(not(target_arch = "wasm32"))]
        {
            std::thread::spawn(move || {
                let file_opt = rfd::FileDialog::new()
                    .add_filter("MPEG Transport Stream", &["ts", "mts", "m2ts"])
                    .add_filter("All Files", &["*"])
                    .pick_file();

                if let Some(path) = file_opt {
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_else(|| path.display().to_string());

                    let result = std::fs::read(&path)
                        .map(|bytes| LoadedFile {
                            name,
                            data: Bytes::from(bytes),
                        })
                        .map_err(|err| format!("Failed to read {}: {err}", path.display()));

                    let _ = tx.send(result);
                    ctx.request_repaint();
                }
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let file_opt = rfd::AsyncFileDialog::new()
                    .add_filter("MPEG Transport Stream", &["ts", "mts", "m2ts"])
                    .add_filter("All Files", &["*"])
                    .pick_file()
                    .await;

                if let Some(file_handle) = file_opt {
                    let name = file_handle.file_name();
                    let bytes = file_handle.read().await;
                    let _ = tx.send(Ok(LoadedFile {
                        name,
                        data: Bytes::from(bytes),
                    }));
                    ctx.request_repaint();
                }
            });
        }
    }

    /// Polls for a completed file load without blocking the UI thread.
    pub(crate) fn poll(&self) -> Option<Result<LoadedFile, String>> {
        self.rx.try_recv().ok()
    }
}

/// A single 188-byte MPEG-TS packet combining parsed [`PacketView`] accessors and raw bytes.
pub(crate) struct TsPacket {
    pub(crate) index: usize,
    pub(crate) view: PacketView,
    pub(crate) raw: Bytes,
}

impl TsPacket {
    /// Byte offset of this packet within the Transport Stream file.
    pub(crate) fn stream_offset(&self) -> usize {
        self.index * PACKET_SIZE
    }

    /// Byte range (`0..4`) corresponding to the 4-byte MPEG-TS header.
    pub(crate) fn header_range(&self) -> Range<usize> {
        0..4.min(self.raw.len())
    }

    /// Byte range within `0..188` corresponding to the Adaptation Field (including its length byte), if present.
    pub(crate) fn adaptation_field_range(&self) -> Option<Range<usize>> {
        if self.raw.len() < 5 {
            return None;
        }
        let af_len = self.raw[4] as usize;
        match self.view.adaptation_field_control() {
            AdaptationFieldControl::AdaptationFieldOnly => {
                let end = (4 + 1 + af_len).min(self.raw.len());
                Some(4..end)
            }
            AdaptationFieldControl::AdaptationFieldAndPayload => {
                let end = (4 + 1 + af_len).min(self.raw.len());
                Some(4..end)
            }
            AdaptationFieldControl::PayloadOnly | AdaptationFieldControl::Reserved => None,
        }
    }

    /// Byte range within `0..188` corresponding to the packet Payload, if present.
    pub(crate) fn payload_range(&self) -> Option<Range<usize>> {
        match self.view.adaptation_field_control() {
            AdaptationFieldControl::PayloadOnly => {
                if self.raw.len() > 4 {
                    Some(4..self.raw.len())
                } else {
                    None
                }
            }
            AdaptationFieldControl::AdaptationFieldAndPayload => {
                if self.raw.len() < 5 {
                    return None;
                }
                let af_len = self.raw[4] as usize;
                let start = 4 + 1 + af_len;
                if start < self.raw.len() {
                    Some(start..self.raw.len())
                } else {
                    None
                }
            }
            AdaptationFieldControl::AdaptationFieldOnly | AdaptationFieldControl::Reserved => None,
        }
    }
}

/// In-memory MPEG Transport Stream container backed by [`bytes::Bytes`].
pub(crate) struct TsContainer {
    name: String,
    buffer: Bytes,
}

impl TsContainer {
    pub(crate) fn from_loaded_file(file: LoadedFile) -> Self {
        Self {
            name: file.name,
            buffer: file.data,
        }
    }

    #[cfg(test)]
    pub(crate) fn new(name: impl Into<String>, buffer: Bytes) -> Self {
        Self {
            name: name.into(),
            buffer,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn byte_len(&self) -> usize {
        self.buffer.len()
    }

    pub(crate) fn len(&self) -> usize {
        self.buffer.len() / PACKET_SIZE
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Quickly reads the 13-bit PID for packet `index` without allocating a [`TsPacket`].
    pub(crate) fn pid_at(&self, index: usize) -> Option<u16> {
        let start = index.checked_mul(PACKET_SIZE)?;
        let end = start.checked_add(PACKET_SIZE)?;
        if end > self.buffer.len() {
            return None;
        }
        let b1 = self.buffer[start + 1] as u16;
        let b2 = self.buffer[start + 2] as u16;
        Some(((b1 & 0b0001_1111) << 8) | b2)
    }

    /// Returns a [`TsPacket`] at `index` with both parsed [`PacketView`] and raw slice.
    pub(crate) fn packet_at(&self, index: usize) -> Option<TsPacket> {
        let start = index.checked_mul(PACKET_SIZE)?;
        let end = start.checked_add(PACKET_SIZE)?;
        if end > self.buffer.len() {
            return None;
        }
        let raw = self.buffer.slice(start..end);
        let view = PacketView::new(raw.clone());
        Some(TsPacket { index, view, raw })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lluvia_media::containers::ts::PACKET_SYNC_BYTE;

    fn make_test_packet(pid: u16, afc: u8, af_len: u8, cc: u8) -> [u8; PACKET_SIZE] {
        let mut pkt = [0xFFu8; PACKET_SIZE];
        pkt[0] = PACKET_SYNC_BYTE;
        pkt[1] = 0b0100_0000 | ((pid >> 8) as u8 & 0x1F); // PUSI = 1
        pkt[2] = (pid & 0xFF) as u8;
        pkt[3] = ((afc & 0b11) << 4) | (cc & 0x0F);
        pkt[4] = af_len;
        if af_len > 0 {
            pkt[5] = 0b0101_0000; // random_access_indicator + pcr_flag
        }
        pkt
    }

    #[test]
    fn test_container_and_packet_ranges() {
        let pkt0 = make_test_packet(0x0000, 0b01, 0, 1); // PayloadOnly
        let pkt1 = make_test_packet(0x0100, 0b11, 7, 2); // AdaptationFieldAndPayload (af_len = 7 -> bytes 4..12)
        let pkt2 = make_test_packet(0x1FFF, 0b10, 183, 3); // AdaptationFieldOnly (af_len = 183 -> bytes 4..188)

        let mut raw_stream = Vec::with_capacity(PACKET_SIZE * 3);
        raw_stream.extend_from_slice(&pkt0);
        raw_stream.extend_from_slice(&pkt1);
        raw_stream.extend_from_slice(&pkt2);

        let container = TsContainer::new("sample.ts", Bytes::from(raw_stream));
        assert_eq!(container.name(), "sample.ts");
        assert_eq!(container.len(), 3);
        assert_eq!(container.byte_len(), PACKET_SIZE * 3);
        assert!(!container.is_empty());

        assert_eq!(container.pid_at(0), Some(0x0000));
        assert_eq!(container.pid_at(1), Some(0x0100));
        assert_eq!(container.pid_at(2), Some(0x1FFF));
        assert_eq!(container.pid_at(3), None);

        let p0 = container.packet_at(0).expect("packet 0");
        assert!(p0.view.is_valid().is_ok());
        assert_eq!(p0.header_range(), 0..4);
        assert_eq!(p0.adaptation_field_range(), None);
        assert_eq!(p0.payload_range(), Some(4..188));

        let p1 = container.packet_at(1).expect("packet 1");
        assert!(p1.view.is_valid().is_ok());
        assert_eq!(p1.header_range(), 0..4);
        assert_eq!(p1.adaptation_field_range(), Some(4..12));
        assert_eq!(p1.payload_range(), Some(12..188));

        let p2 = container.packet_at(2).expect("packet 2");
        assert!(p2.view.is_valid().is_ok());
        assert_eq!(p2.header_range(), 0..4);
        assert_eq!(p2.adaptation_field_range(), Some(4..188));
        assert_eq!(p2.payload_range(), None);
    }
}
