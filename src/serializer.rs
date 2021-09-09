use codec::{Compact, Encode};
use core::fmt;
use core2::io;
use serde::{ser, Serialize};

type Result<T> = core::result::Result<T, Error>;

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    T: Serialize,
    W: io::Write,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(())
}

///
pub struct Serializer<W>(W);

impl<W: io::Write> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Serializer(writer)
    }
}

impl<'a, W: io::Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok(v.encode_to(&mut self.0))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Ok(v.encode_to(&mut self.0))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Ok(v.encode_to(&mut self.0))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(v.encode_to(&mut self.0))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(v.encode_to(&mut self.0))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.0.write(&[v])?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        Ok(v.encode_to(&mut self.0))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        Ok(v.encode_to(&mut self.0))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        Ok(v.encode_to(&mut self.0))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(v.encode_to(&mut self.0))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.0.write_all(v).map_err(Error::from)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.0.write(&[0x00])?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        self.0.write(&[0x01])?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        __name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        (variant_index as u8).serialize(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        __name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Compact(len.expect("known length") as u64).encode_to(&mut self.0);
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        __name: &'static str,
        __len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        __name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        __len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        __name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        __len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'a, W> ser::SerializeMap for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a, W> ser::SerializeSeq for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a, W> ser::SerializeStructVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeTupleStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

#[derive(Debug)]
pub enum Error {}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl ser::StdError for Error {}

impl ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: fmt::Display,
    {
        todo!()
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn test_primitive_u8() -> Result<()> {
        let mut out = [0u8];
        to_writer(&mut out[..], &123u8)?;

        let expected = [123];

        assert_eq!(out, expected);
        Ok(())
    }

    #[test]
    fn test_primitive_u16() -> Result<()> {
        const INPUT: u16 = 0xFF_EE;
        let mut out = [0u8; size_of::<u16>()];
        let expected = INPUT.encode();

        to_writer(out.as_mut(), &INPUT)?;

        assert_eq!(out.as_ref(), expected);
        Ok(())
    }

    #[test]
    fn test_primitive_u32() -> Result<()> {
        const INPUT: u32 = 0xFF_EE_DD_CC;
        let mut out = [0u8; size_of::<u32>()];
        let expected = INPUT.encode();

        to_writer(out.as_mut(), &INPUT)?;

        assert_eq!(out.as_ref(), expected);
        Ok(())
    }

    #[test]
    fn test_primitive_u64() -> Result<()> {
        const INPUT: u64 = 0xFF_EE_DD_CC__BB_AA_99_88;
        let mut out = [0u8; size_of::<u64>()];
        let expected = INPUT.encode();

        to_writer(out.as_mut(), &INPUT)?;

        assert_eq!(out.as_mut(), expected);
        Ok(())
    }

    #[test]
    fn test_primitive_i16() -> Result<()> {
        const INPUT: i16 = i16::MIN;
        let mut out = [0u8; size_of::<i16>()];
        let expected = INPUT.encode();

        to_writer(out.as_mut(), &INPUT)?;

        assert_eq!(out.as_mut(), expected);
        Ok(())
    }

    #[test]
    fn test_primitive_i32() -> Result<()> {
        const INPUT: i32 = i32::MIN;
        let mut out = [0u8; size_of::<i32>()];
        let expected = INPUT.encode();

        to_writer(out.as_mut(), &INPUT)?;

        assert_eq!(out.as_mut(), expected);
        Ok(())
    }

    #[test]
    fn test_primitive_i64() -> Result<()> {
        const INPUT: i64 = i64::MIN;
        let mut out = [0u8; size_of::<i64>()];
        let expected = INPUT.encode();

        to_writer(out.as_mut(), &INPUT)?;

        assert_eq!(out.as_mut(), expected);
        Ok(())
    }

    #[test]
    fn test_primitive_bool() -> Result<()> {
        const INPUT: bool = true;
        let mut out = [0u8];
        let expected = INPUT.encode();

        to_writer(out.as_mut(), &INPUT)?;

        assert_eq!(out.as_mut(), expected);
        Ok(())
    }

    #[test]
    fn test_str() -> Result<()> {
        const INPUT: &str = "ac orci phasellus egestas tellus rutrum tellus pellentesque";
        let mut out = Vec::<u8>::new();
        let expected = INPUT.encode();

        to_writer(&mut out, &INPUT)?;

        assert_eq!(out, expected);
        Ok(())
    }

    #[test]
    fn test_bytes() -> Result<()> {
        const INPUT: &[u8] = b"dictumst quisque sagittis purus sit amet volutpat consequat";
        let mut out = Vec::<u8>::new();
        let expected = INPUT.encode();

        to_writer(&mut out, &INPUT)?;

        assert_eq!(out, expected);
        Ok(())
    }

    #[test]
    fn test_tuple_simple() -> Result<()> {
        const INPUT: (u8, bool, u64) = (0xD0, false, u64::MAX);
        let mut out = Vec::<u8>::new();
        let expected = INPUT.encode();

        to_writer(&mut out, &INPUT)?;

        assert_eq!(out, expected);
        Ok(())
    }

    #[test]
    fn test_enum_simple() -> Result<()> {
        #[derive(Serialize, Encode)]
        enum X {
            _A,
            B,
        }

        const INPUT: X = X::B;
        let mut out = Vec::<u8>::new();
        let expected = INPUT.encode();

        to_writer(&mut out, &INPUT)?;

        assert_eq!(out, expected);
        Ok(())
    }

    #[test]
    fn test_tuple_enum_mix() -> Result<()> {
        #[derive(Serialize, Encode)]
        enum X {
            A,
            B,
        }

        let input: (Option<()>, Option<String>, X, X) = (None, Some("hello".into()), X::A, X::B);
        let mut out = Vec::<u8>::new();
        let expected = input.encode();

        to_writer(&mut out, &input)?;

        assert_eq!(out, expected);
        Ok(())
    }
}