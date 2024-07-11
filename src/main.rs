mod chunk_type;
mod chunk;


fn main() {
    let x25: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);
    println!("{}", x25.checksum(&[0,12,3,4,56,67,78,98,97,89,89,89,89,89]));
}
