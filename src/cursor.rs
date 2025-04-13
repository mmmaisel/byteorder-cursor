#[cfg(not(feature = "std"))]
use core::{fmt::Debug, prelude::rust_2021::derive, result::Result::Ok};

use byteorder::ByteOrder;

use super::BufferTooSmall;

/// A std::io::Cursor like buffer interface with byteorder support and no_std
/// compatibility.
#[derive(Debug)]
pub struct Cursor<T> {
    buffer: T,
    pos: usize,
}

impl<T: AsRef<[u8]>> Cursor<T> {
    /// Constructs a new cursor object on top of a slice.
    pub fn new(buffer: T) -> Self {
        Self { buffer, pos: 0 }
    }

    #[allow(clippy::len_without_is_empty)]
    /// Returns the length of the underlying buffer.
    pub fn len(&self) -> usize {
        self.buffer.as_ref().len()
    }

    /// Returns the remaining length in bytes of the underlying buffer.
    pub fn remaining(&self) -> usize {
        self.buffer.as_ref().len() - self.pos
    }

    /// Checks if the underlying buffer has the expected amount of space left.
    pub fn check_remaining(
        &self,
        expected: usize,
    ) -> Result<(), BufferTooSmall> {
        if self.remaining() < expected {
            return Err(BufferTooSmall {
                size: self.len(),
                expected: self.pos + expected,
            });
        }

        Ok(())
    }

    /// Returns the cursor position in the underlying buffer.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Sets the cursor position in the underlying buffer.
    pub fn set_position(&mut self, position: usize) {
        self.pos = position
    }

    /// Advances it cursor position by the given amount of bytes.
    pub fn skip(&mut self, count: usize) {
        self.pos += count;
    }

    /// Reads data from the underlying buffer to the given slice and advances
    /// cursor position.
    /// Panics if there is not enough data remaining to fill the slice.
    pub fn read_bytes(&mut self, dst: &mut [u8]) {
        dst.copy_from_slice(
            &self.buffer.as_ref()[self.pos..(self.pos + dst.len())],
        );
        self.pos += dst.len();
    }

    /// Reads a 8bit integer value from the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough data remaining.
    pub fn read_u8(&mut self) -> u8 {
        let val = self.buffer.as_ref()[self.pos];
        self.pos += 1;
        val
    }

    /// Reads a 16bit integer value from the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough data remaining.
    pub fn read_u16<B: ByteOrder>(&mut self) -> u16 {
        let val = B::read_u16(&self.buffer.as_ref()[self.pos..]);
        self.pos += 2;
        val
    }

    /// Reads a 24bit integer value from the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough data remaining.
    pub fn read_u24<B: ByteOrder>(&mut self) -> u32 {
        let val = B::read_u24(&self.buffer.as_ref()[self.pos..]);
        self.pos += 3;
        val
    }

    /// Reads a 32bit integer value from the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough data remaining.
    pub fn read_u32<B: ByteOrder>(&mut self) -> u32 {
        let val = B::read_u32(&self.buffer.as_ref()[self.pos..]);
        self.pos += 4;
        val
    }

    /// Reads a 64bit integer value from the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough data remaining.
    pub fn read_u64<B: ByteOrder>(&mut self) -> u64 {
        let val = B::read_u64(&self.buffer.as_ref()[self.pos..]);
        self.pos += 8;
        val
    }

    /// Reads a 8bit integer value from the underlying buffer at a given
    /// offset from the cursor position without advancing the cursor position.
    /// Panics if there is not enough data remaining.
    pub fn peek_u8(&self, offset: usize) -> u8 {
        self.buffer.as_ref()[self.pos + offset]
    }

    /// Reads a 16bit integer value from the underlying buffer at a given
    /// offset from the cursor position without advancing the cursor position.
    /// Panics if there is not enough data remaining.
    pub fn peek_u16<B: ByteOrder>(&self, offset: usize) -> u16 {
        B::read_u16(&self.buffer.as_ref()[(self.pos + offset)..])
    }

    /// Reads a 24bit integer value from the underlying buffer at a given
    /// offset from the cursor position without advancing the cursor position.
    /// Panics if there is not enough data remaining.
    pub fn peek_u24<B: ByteOrder>(&self, offset: usize) -> u32 {
        B::read_u24(&self.buffer.as_ref()[(self.pos + offset)..])
    }

    /// Reads a 32bit integer value from the underlying buffer at a given
    /// offset from the cursor position without advancing the cursor position.
    /// Panics if there is not enough data remaining.
    pub fn peek_u32<B: ByteOrder>(&self, offset: usize) -> u32 {
        B::read_u32(&self.buffer.as_ref()[(self.pos + offset)..])
    }
}

impl Cursor<&mut [u8]> {
    /// Writes the given slice to the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough space remaining to write the slice.
    pub fn write_bytes(&mut self, src: &[u8]) {
        self.buffer[self.pos..(self.pos + src.len())].copy_from_slice(src);
        self.pos += src.len();
    }

    /// Writes a 8bit integer value to the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough space remaining.
    pub fn write_u8(&mut self, val: u8) {
        self.buffer[self.pos] = val;
        self.pos += 1;
    }

    /// Writes a 16bit integer value to the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough space remaining.
    pub fn write_u16<B: ByteOrder>(&mut self, val: u16) {
        B::write_u16(&mut self.buffer[self.pos..], val);
        self.pos += 2;
    }

    /// Writes a 24bit integer value to the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough space remaining.
    pub fn write_u24<B: ByteOrder>(&mut self, val: u32) {
        B::write_u24(&mut self.buffer[self.pos..], val);
        self.pos += 3;
    }

    /// Writes a 32bit integer value to the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough space remaining.
    pub fn write_u32<B: ByteOrder>(&mut self, val: u32) {
        B::write_u32(&mut self.buffer[self.pos..], val);
        self.pos += 4;
    }

    /// Writes a 64bit integer value to the underlying buffer and advances
    /// cursor position.
    /// Panics if there is not enough space remaining.
    pub fn write_u64<B: ByteOrder>(&mut self, val: u64) {
        B::write_u64(&mut self.buffer[self.pos..], val);
        self.pos += 8;
    }
}

#[cfg(test)]
mod tests {
    use byteorder::{BigEndian, LittleEndian};

    use super::*;

    #[test]
    fn test_read() {
        let buffer = [1u8, 0, 1, 0, 1, 0, 0, 0, 0, 2, 2, 0, 0, 0];
        let mut cursor = Cursor::new(&buffer[..]);

        assert_eq!(cursor.len(), 14);
        assert_eq!(cursor.remaining(), 14);

        assert_eq!(cursor.read_u8(), 1);
        assert_eq!(cursor.remaining(), 13);

        assert_eq!(cursor.read_u16::<BigEndian>(), 1);
        cursor.skip(1);
        assert_eq!(cursor.read_u16::<LittleEndian>(), 1);
        assert!(cursor.check_remaining(8).is_ok());

        assert_eq!(cursor.read_u32::<BigEndian>(), 2);
        assert_eq!(cursor.read_u32::<LittleEndian>(), 2);
        assert!(cursor.check_remaining(1).is_err());

        cursor.set_position(1);
        let mut buf = [0u8; 4];
        cursor.read_bytes(&mut buf);
        assert_eq!(buf, [0, 1, 0, 1]);
    }

    #[test]
    fn test_peek() {
        let buffer = [1u8, 0, 1, 0, 1, 0, 0, 0, 0, 2, 2, 0, 0, 0];
        let mut cursor = Cursor::new(&buffer[..]);

        assert_eq!(cursor.peek_u8(1), 0);
        assert_eq!(cursor.peek_u16::<BigEndian>(2), 256);
        assert_eq!(cursor.peek_u32::<LittleEndian>(6), 0x02000000);

        cursor.set_position(5);
        assert_eq!(cursor.peek_u8(1), 0);
        assert_eq!(cursor.peek_u16::<BigEndian>(3), 2);
        assert_eq!(cursor.peek_u32::<LittleEndian>(5), 2);
    }

    #[test]
    fn test_write() {
        let mut buffer = [0u8; 16];
        let mut cursor = Cursor::new(&mut buffer[..]);

        cursor.write_u8(1);
        cursor.write_u16::<BigEndian>(5);
        cursor.write_u32::<LittleEndian>(16);
        cursor.write_bytes(&[1, 2, 3, 4, 5]);

        assert_eq!(cursor.position(), 12);
        assert_eq!(buffer, [1, 0, 5, 16, 0, 0, 0, 1, 2, 3, 4, 5, 0, 0, 0, 0]);
    }
}
