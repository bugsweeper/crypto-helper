use crate::length::{read_len, write_len};
use crate::reader::{read_data, Reader};
use crate::writer::Writer;
use crate::{Asn1, Asn1Decode, Asn1Entity, Asn1Result, Asn1Type, Error, Tag, Asn1Encode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bool {
    flag: bool,
}

impl Bool {
    pub const TAG: Tag = Tag(1);
}

impl From<bool> for Bool {
    fn from(flag: bool) -> Self {
        Self { flag }
    }
}

impl TryFrom<u8> for Bool {
    type Error = Error;

    fn try_from(flag: u8) -> Result<Self, Self::Error> {
        match flag {
            0 => Ok(Self { flag: false }),
            0xff => Ok(Self { flag: true }),
            _ => Err(Error::from("Invalid bool value")),
        }
    }
}

impl Asn1Entity for Bool {
    fn tag(&self) -> &Tag {
        &Self::TAG
    }
}

impl<'data> Asn1Decode<'data> for Bool {
    fn compare_tags(tag: &Tag) -> bool {
        Self::TAG == *tag
    }

    fn decode(reader: &mut Reader<'_>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        if len != 1 {
            return Err(Error::from("Bool length must be equal to 1"));
        }

        Ok(reader.read_byte()?.try_into()?)
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;

        if len != 1 {
            return Err(Error::from("Bool length must be equal to 1"));
        }

        let (data, data_range) = read_data(reader, len)?;

        Ok(Asn1 {
            raw_data: reader.data_in_range(tag_position..data_range.end)?,
            tag: tag_position,
            length: len_range,
            data: data_range,
            asn1_type: Asn1Type::Bool(data[0].try_into()?),
        })
    }
}

impl Asn1Encode for Bool {
    fn needed_buf_size(&self) -> usize {
        1 /* tag */ + 1 /* len */ + 1 /* bool value */
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(1, writer)?;
        writer.write_byte(match self.flag {
            true => 0xff,
            false => 0
        })
    }
}
