use std::fs::{File, OpenOptions};
use std::io::{self, Cursor, Read, Seek};

use byteorder::{NativeEndian, ReadBytesExt};

#[derive(Default)]
pub struct ImageFile {
    name: String,
    file: Option<File>,
    file_size: u64,
    pub header: Header,
    index: Index,
}

/// Image File Header
#[derive(Default, Debug)]
pub struct Header {
    /// File Identifier
    magic: u32,
    /// Major, Minor Version
    /// Major = version >> 16
    /// Minor = version & 0xFFFF
    version: u32,
    /// Image Wide flags (unused?)
    flags: u32,
    /// Number of resources in file
    resource_count: u32,
    /// Length of lookup tables used in index
    table_length: u32,
    /// Number of bytes in the region used to store location attibute streams
    attributes_size: u32,
    /// Size of the region used to store strings used by the index and meta data
    strings_size: u32,
}

/**
 * Index contains information related to resource lookup.
 * The algorithm used for lookup is "A Practical Minimal Perfect Hashing Method"
 *  (http://homepages.dcc.ufmg.br/~nivio/papers/wea05.pdf).
 */
#[derive(Default)]
pub struct Index {
    size: usize,

    /**
     * Array of 32-bit signed values representing actions
     *    that should take place for hashed strings that map to that value.
     * Negative Values indicate no hash collision and can be
     *    quickly converted to indices into attribute offsets/
     * Positive values represent a new seed for hashing an index into attribute offsets.
     *   Zero indicates not found.
     */
    redirect_table: Vec<i32>,
    /**
     * Array of 32-bit unsigned values representing offsets into attribute data.
     * Attribute offsets can be iterated to do a full survey of resources in the image.
     * Offset of zero indicates no attributes.
     */
    attribute_offsets: Vec<u32>,
    /// Bytes representing compact attribute data for attributes.
    attribute_data: Vec<u8>,
    /**
     * Collection of zero terminated UTF-8 strings used by the index and image meta data.
     * Eachg string accessed by offset.
     * Each string is unique.
     * Offset zero is reserved for the empty string.
     */
    strings: Vec<String>,
}

struct Resource {}

impl ImageFile {
    // Read-only access
    pub fn open(name: &str) -> io::Result<ImageFile> {
        let mut image = ImageFile::default();

        let mut file = File::open(name)?;

        let file_size = file.metadata()?.len();
        image.file_size = file_size;

        if (file_size as usize) < size_of::<Header>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "File Size was smaller than Header Size",
            ));
        }

        let mut header_buffer = [0u8; size_of::<Header>()];
        let read = file.read(&mut header_buffer[..])?;

        assert_eq!(size_of::<Header>(), read);

        let header = Header::from_bytes(&header_buffer[..])?;

        if header.magic != Header::IMAGE_MAGIC
            || header.get_major_minor()
                != (Header::MAJOR_VERSION, Header::MINOR_VERSION)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Header information was incorrect",
            ));
        }
        image.header = header;

        if (file_size as usize) < Index::index_size(&image.header) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "File Size was smaller than Header Size",
            ));
        }

        let mut index_data = vec![0u8; Index::index_size(&image.header)];
        let read = file.read(&mut index_data)?;

        assert_eq!(Index::index_size(&image.header), read);

        let index = Index::from_bytes(&image.header, &index_data)?;

        Ok(image)
    }
}

impl Header {
    pub const IMAGE_MAGIC: u32 = 0xCAFEDADA;
    pub const IMAGE_MAGIC_INVERT: u32 = 0xDADAFECA;
    pub const MAJOR_VERSION: u32 = 1;
    pub const MINOR_VERSION: u32 = 0;

    pub fn from_bytes(bytes: &[u8]) -> io::Result<Header> {
        let mut cursor = Cursor::new(bytes);
        let header = Header {
            magic: cursor.read_u32::<NativeEndian>()?,
            version: cursor.read_u32::<NativeEndian>()?,
            flags: cursor.read_u32::<NativeEndian>()?,
            resource_count: cursor.read_u32::<NativeEndian>()?,
            table_length: cursor.read_u32::<NativeEndian>()?,
            attributes_size: cursor.read_u32::<NativeEndian>()?,
            strings_size: cursor.read_u32::<NativeEndian>()?,
        };
        assert_eq!(cursor.position() as usize, bytes.len());
        Ok(header)
    }

    pub fn get_major_minor(&self) -> (u32, u32) {
        (self.version >> 16, self.version & 0xFFFF)
    }
}

impl Index {
    #[inline]
    fn index_size(header: &Header) -> usize {
        size_of::<Header>()
            + header.table_length as usize * size_of::<u32>() * 2
            + header.attributes_size as usize
            + header.strings_size as usize
    }

    pub fn from_bytes(header: &Header, bytes: &[u8]) -> io::Result<Index> {
        let mut cursor = Cursor::new(bytes);
        let length = header.table_length as usize;
        // Table starts at the beginning of header, the bytes we're reading is already beyond header
        let offsets_table_offset = length * size_of::<i32>();
        let attribute_bytes_offset =
            offsets_table_offset + length * size_of::<u32>();
        let string_bytes_offset =
            attribute_bytes_offset + header.attributes_size as usize;

        let mut redirect_table = vec![0i32; header.table_length as usize];
        for i in 0..redirect_table.capacity() {
            redirect_table[i] = cursor.read_i32::<NativeEndian>()?;
        }

        let offset_table_size = (offsets_table_offset
            + length / size_of::<u32>())
            - offsets_table_offset;

        let mut attribute_offsets = vec![0u32; offset_table_size];
        for i in 0..attribute_offsets.capacity() {
            attribute_offsets[i] = cursor.read_u32::<NativeEndian>()?;
        }

        let mut attribute_data = vec![0u8; header.attributes_size as usize];
        let read = cursor.read(&mut attribute_data)?;
        assert_eq!(read, header.attributes_size as usize);

        let mut strings: Vec<String> = vec![];
        let mut cur_string: Vec<u8> = vec![];
        for _ in 0..header.strings_size {
            let next = cursor.read_u8()?;
            if next == b'\0' {
                strings.push(String::from_utf8(cur_string).unwrap());
                cur_string = vec![];
            }
            cur_string.push(next);
        }

        Ok(Index {
            size: Index::index_size(header),
            redirect_table,
            attribute_offsets,
            attribute_data,
            strings,
        })
    }
}
