// use memmap2::{MmapMut, MmapOptions};
// use tokio::{fs::File, io::AsyncWriteExt};

// pub const SECTOR_SIZE: usize = 4096;

// pub struct SectorFs {
//     file: File,
//     pub mmap: MmapMut,
// }

// // impl SectorFs {
//     pub fn new(file: File) -> Self {
//         let mmap = unsafe { MmapOptions::new().map_mut(&file).unwrap() };
//         Self { file, mmap }
//     }

//     pub fn write(&mut self, sector: u64, data: &[u8]) {
//         let mut offset = sector * SECTOR_SIZE as u64;

//         // Jika ukuran data lebih besar dari sektor, pecah data ke sektor-sektor berturut-turut
//         let mut data_remaining = data;
//         while !data_remaining.is_empty() {
//             // Menghitung panjang data yang akan ditulis pada sektor ini
//             let len_to_copy = data_remaining.len().min(SECTOR_SIZE);

//             // Membuat buffer dengan padding (0x00)
//             let mut buffer = vec![0u8; SECTOR_SIZE];
//             buffer[0..len_to_copy].copy_from_slice(&data_remaining[0..len_to_copy]);

//             // Menulis buffer ke mmap
//             let start_index = (offset as usize) % self.mmap.len();
//             let end_index = start_index + SECTOR_SIZE;
//             let mmap_slice: &mut [u8] = &mut self.mmap[start_index..end_index];
//             mmap_slice.copy_from_slice(&buffer);

//             // Sinkronisasi perubahan ke disk setelah menulis sektor

//             // Memperbarui data yang tersisa untuk ditulis
//             data_remaining = &data_remaining[len_to_copy..];

//             // Memperbarui offset untuk sektor berikutnya
//             offset += SECTOR_SIZE as u64;
//         }
//         self.mmap.flush().unwrap();
//     }

//     pub async fn close(&mut self) {
//         self.mmap.flush().unwrap();
//         self.file.sync_data().await.unwrap();
//         self.file.sync_all().await.unwrap();
//         self.file.shutdown().await.unwrap();
//     }
// }
