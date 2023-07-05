use alloc::vec::Vec;

use crate::length::{len_size, read_len, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1, Asn1Decode, Asn1Encode, Asn1Entity, Asn1Result, Asn1Type, Tag};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Sequence<'data> {
    fields: Vec<Asn1<'data>>,
}

pub type OwnedSequence = Sequence<'static>;

impl Sequence<'_> {
    pub const TAG: Tag = Tag(0x30);
}

impl Asn1Entity for Sequence<'_> {
    fn tag(&self) -> &Tag {
        &Self::TAG
    }
}

impl Asn1Encode for Sequence<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.fields.iter().map(|f| f.asn1().needed_buf_size()).sum();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;

        let data_len = self.fields.iter().map(|f| f.asn1().needed_buf_size()).sum();
        write_len(data_len, writer)?;

        self.fields.iter().try_for_each(|f| f.asn1().encode(writer))
    }
}

impl<'data> Asn1Decode<'data> for Sequence<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        &Self::TAG == tag
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        let mut fields = Vec::new();

        let position = reader.position();
        while reader.position() - position < len {
            fields.push(Asn1Type::decode_asn1(reader)?);
        }

        Ok(Self { fields })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;
        let data_range = len_range.end..len_range.end + len;

        let mut fields = Vec::new();

        let position = reader.position();
        while reader.position() - position < len {
            fields.push(Asn1Type::decode_asn1(reader)?);
        }

        Ok(Asn1 {
            raw_data: reader.data_in_range(tag_position..len_range.end + len)?,
            tag: tag_position,
            length: len_range,
            data: data_range,
            asn1_type: Asn1Type::Sequence(Sequence { fields }),
        })
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::{Asn1, Asn1Decode, Asn1Type, OctetString, Sequence, Utf8String};

    #[test]
    fn example() {
        let raw = [
            48, 27, 4, 8, 0, 17, 34, 51, 68, 85, 102, 119, 12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114,
            121, 110, 107, 97,
        ];

        let decoded = Sequence::decode_asn1_buff(&raw).unwrap();

        assert_eq!(
            decoded,
            Asn1 {
                raw_data: &raw,
                tag: 0,
                length: 1..2,
                data: 2..29,
                asn1_type: Asn1Type::Sequence(Sequence {
                    fields: vec![
                        Asn1 {
                            raw_data: &[4, 8, 0, 17, 34, 51, 68, 85, 102, 119],
                            tag: 2,
                            length: 3..4,
                            data: 4..12,
                            asn1_type: Asn1Type::OctetString(OctetString::from(vec![0, 17, 34, 51, 68, 85, 102, 119]))
                        },
                        Asn1 {
                            raw_data: &[12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114, 121, 110, 107, 97],
                            tag: 12,
                            length: 13..14,
                            data: 14..29,
                            asn1_type: Asn1Type::Utf8String(Utf8String::from("thebesttvarynka"))
                        },
                    ]
                }),
            }
        );
    }
}
