use byteorder_cursor::{BigEndian, BufferTooSmall, Cursor, LittleEndian};

fn main() -> Result<(), BufferTooSmall> {
    let mut buffer = [0u8; 16];
    let mut cursor = Cursor::new(&mut buffer[..]);

    cursor.write_u8(1);
    cursor.write_u16::<BigEndian>(5);
    cursor.write_u32::<LittleEndian>(16);
    cursor.write_bytes(&[1, 2, 3, 4, 5]);

    println!("Remaining space in cursor is {} bytes", cursor.remaining());
    cursor.set_position(0);
    cursor.check_remaining(7)?;

    let x = cursor.read_u8();
    let y = cursor.read_u16::<BigEndian>();
    let z = cursor.read_u32::<LittleEndian>();

    let mut w = [0u8; 5];
    cursor.read_bytes(&mut w);

    println!("Read-back from buffer: {x}, {y}, {z}, {w:?}");
    println!("Cursor is now at position {}", cursor.position());

    Ok(())
}
