use alloc::borrow::Cow;
use alloc::vec::Vec;

use crate::length::{len_size, read_len, write_len};
use crate::reader::{read_data, Reader};
use crate::writer::Writer;
use crate::{Asn1, Asn1Decode, Asn1Encode, Asn1Entity, Asn1Result, Asn1Type, Tag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OctetString<'data> {
    octets: Cow<'data, [u8]>,
}

pub type OwnedOctetString = OctetString<'static>;

impl OctetString<'_> {
    pub const TAG: Tag = Tag(4);
}

impl From<Vec<u8>> for OwnedOctetString {
    fn from(data: Vec<u8>) -> Self {
        Self {
            octets: Cow::Owned(data),
        }
    }
}

impl<'data> Asn1Decode<'data> for OctetString<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        OctetString::TAG == *tag
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        let (data, _data_range) = read_data(reader, len)?;

        Ok(Self {
            octets: Cow::Borrowed(data),
        })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;

        let (data, data_range) = read_data(reader, len)?;

        Ok(Asn1 {
            raw_data: reader.data_in_range(tag_position..data_range.end)?,
            tag: tag_position,
            length: len_range,
            data: data_range,
            asn1_type: Asn1Type::OctetString(Self {
                octets: Cow::Borrowed(data),
            }),
        })
    }
}

impl Asn1Entity for OctetString<'_> {
    fn tag(&self) -> &Tag {
        &OctetString::TAG
    }
}

impl Asn1Encode for OctetString<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.octets.len();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.octets.len(), writer)?;
        writer.write_slice(&self.octets)
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::Reader;
    use crate::{Asn1Decode, Asn1Encode, OctetString};

    #[test]
    fn example() {
        let raw = [4, 8, 0, 17, 34, 51, 68, 85, 102, 119];

        let octet_string = OctetString::decode_asn1(&mut Reader::new(&raw)).unwrap();

        assert_eq!(octet_string.tag_position(), 0);
        assert_eq!(octet_string.length_bytes(), &[8]);
        assert_eq!(octet_string.length_range(), 1..2);
        assert_eq!(&raw[octet_string.data_range()], &[0, 17, 34, 51, 68, 85, 102, 119]);

        let mut encoded = [0; 10];

        assert_eq!(octet_string.asn1().needed_buf_size(), 10);

        octet_string.asn1().encode_buff(&mut encoded).unwrap();

        assert_eq!(encoded, raw);
    }
}
