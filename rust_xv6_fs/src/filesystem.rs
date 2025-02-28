// src/filesystem.rs

use std::collections::VecDeque;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Superblock {
    pub size: u32,
    pub block_size: u32,
    pub inode_count: u32,
    pub free_inode_count: u32,
    pub free_block_count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Inode {
    pub type_: u16,
    pub size: u32,
    pub block_pointers: [u32; 12],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dirent {
    pub inode: u32,
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DataBlock {
    pub data: [u8; 512],
}

pub struct FileSystem {
    pub superblock: Superblock,
    pub inodes: Vec<Inode>,
    pub data_blocks: Vec<DataBlock>,
    pub dirents: Vec<Dirent>,
}

impl FileSystem {
    pub fn new(size: u32, block_size: u32, inode_count: u32) -> Self {
        let superblock = Superblock {
            size,
            block_size,
            inode_count,
            free_inode_count: inode_count,
            free_block_count: size / block_size,
        };

        FileSystem {
            superblock,
            inodes: Vec::new(),
            data_blocks: Vec::new(),
            dirents: Vec::new(),
        }
    }

    pub fn create_file(&mut self, name: &str) -> Result<u32, &'static str> {
        if self.superblock.free_inode_count == 0 {
            return Err("No free inodes available");
        }

        let inode_index = self
            .inodes
            .iter_mut()
            .position(|inode| inode.type_ == 0)
            .ok_or("No free inodes")?;

        let mut inode = Inode {
            type_: 1,
            size: 0,
            block_pointers: [0; 12],
        };

        let data_block_index = self
            .data_blocks
            .iter_mut()
            .position(|block| block.data == [0; 512])
            .ok_or("No free data blocks")?;

        inode.block_pointers[0] = data_block_index as u32;

        self.superblock.free_inode_count -= 1;
        self.superblock.free_block_count -= 1;

        let mut dirent = Dirent {
            inode: inode_index as u32,
            name: [0; 32],
        };
        let name_bytes = name.as_bytes();
        dirent.name[..name_bytes.len()].copy_from_slice(name_bytes);
        self.dirents.push(dirent);

        self.inodes.push(inode);
        Ok(inode_index as u32)
    }

    pub fn read_file(&self, inode_index: u32) -> Option<&[u8]> {
        let inode = &self.inodes[inode_index as usize];
        if inode.type_ == 0 {
            return None;
        }

        let block_index = inode.block_pointers[0] as usize;
        Some(&self.data_blocks[block_index].data)
    }

    pub fn write_file(&mut self, inode_index: u32, data: &[u8]) -> Result<(), &'static str> {
        let inode = &mut self.inodes[inode_index as usize];
        if inode.type_ == 0 {
            return Err("Not a file");
        }

        let block_index = inode.block_pointers[0] as usize;
        let data_block = &mut self.data_blocks[block_index];

        if data.len() > data_block.data.len() {
            return Err("Data too large for block");
        }

        data_block.data[..data.len()].copy_from_slice(data);
        inode.size = data.len() as u32;

        Ok(())
    }

    pub fn list_files(&self) {
        for dirent in &self.dirents {
            let name = core::str::from_utf8(&dirent.name).unwrap_or("");
            println!("File: {} (inode: {})", name, dirent.inode);
        }
    }
}

pub fn u32_to_string(val: u32) -> String {
    val.to_string()
}
