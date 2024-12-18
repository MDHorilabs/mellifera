use futures::future::{join_all, Future};
use memmap2::{MmapMut, MmapOptions};
use std::io::{Error, ErrorKind};
use tokio::fs::File;

pub const SECTOR_SIZE: usize = 4096;

pub struct Sectors {
    file: File,
    pub mmap: MmapMut,
}

impl Sectors {
    pub fn new(file: File) -> Self {
        let mmap = unsafe { MmapOptions::new().map_mut(&file).unwrap() };
        Self { file, mmap }
    }

    pub async fn get_one(&self, offset: usize) -> [u8; SECTOR_SIZE] {
        self.mmap[offset..(offset as usize + SECTOR_SIZE)]
            .try_into()
            .unwrap()
    }

    pub async fn get_ranges(&self, start: u64, end: u64) -> Vec<[u8; SECTOR_SIZE]> {
        let mut futures = Vec::new();
        for i in start..end {
            futures.push(self.get_one(i as usize));
        }
        join_all(futures).await
    }

    pub async fn write_one(&mut self, offset: usize, data: &[u8]) {
        let mut buff = [0u8; SECTOR_SIZE];
        buff[0..data.len()].copy_from_slice(data);
        self.mmap[offset..(offset + SECTOR_SIZE)].copy_from_slice(&buff);
    }

    pub async fn write_ranges(
        &mut self,
        offset: usize,
        end: usize,
        data: &[u8],
    ) -> Result<(), Error> {
        let start = offset * SECTOR_SIZE;
        let length = end * SECTOR_SIZE;
        let end = start + length;
        let len_to_copy = &data.len().min(length);
        if len_to_copy > &length {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Data length is too large for the specified range",
            ));
        }
        if self.mmap[end + 8].to_le() != 0 || self.mmap[end + 7].to_le() != 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "The specified range is already written",
            ));
        }
        let mut buffer = vec![0u8; length];
        buffer[0..len_to_copy.to_le()].copy_from_slice(&data[0..len_to_copy.to_le()]);

        self.mmap[start..end].copy_from_slice(&buffer);
        self.flush().await;
        Ok(())
    }

    pub async fn flush(&mut self) {
        self.mmap.flush().unwrap();
        self.file.sync_data().await.unwrap();
    }
}
