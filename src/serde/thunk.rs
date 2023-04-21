use base64::{DecodeError, URL_SAFE};
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{ser::Error as _, Deserialize, Serialize, Serializer};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    num::ParseIntError,
    str::Utf8Error,
    string::FromUtf8Error,
};

/// Enum modelling the different errors that can occur during processing of a [`Thunk`]
///
/// ## Why is this a seperate enum
/// One might wonder why this enum exists, and why we don't simply reuse
/// [`Error`](::serde::de::error::Error). The main reason is that I do not want to include variants
/// in that enum that do not occur during the actual deserialization phase. The second reason has to
/// do with lifetimes: Just using `Error<'a>` for the return type in the [`ThunkContent`] functions
/// used by [`Thunk`] is not possible. The reason for that is that processing errors are returned in
/// contexts where data is transformed into owned representations. This means we cannot simply reuse
/// the lifetime the input data is bound to for our errors, as the errors potentially have to
/// outlive the input data (in the worst case they have to be `'static`). Adding a new lifetime to
/// `Thunk` just to use that for the error type is obviously impractical, however it is possible to
/// use `Error<'static>`, which at least doesn't add more downsides. However it still leaves us with
/// an error enum dealing with too much stuff.
#[derive(Debug)]
pub enum ProcessError {
    /// Some utf8 encoding error occurred during processing
    Utf8(Utf8Error),

    /// Some utf8 encoding error occurred while processing after some backing storage was allocated
    FromUtf8(FromUtf8Error),

    /// Some base64 decoding error occurred during processing
    Base64(DecodeError),

    /// Some error occurred when parsing a number
    IntParse(ParseIntError),
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::Utf8(utf8) => utf8.fmt(f),
            ProcessError::Base64(decode) => decode.fmt(f),
            ProcessError::IntParse(int) => int.fmt(f),
            ProcessError::FromUtf8(from_utf8) => from_utf8.fmt(f),
        }
    }
}

impl std::error::Error for ProcessError {}

/// Input value whose further deserialization has been delayed
///
/// This is often used if further processing would require an allocation (for instance when using
/// base64 decoding) or be very long (for instance parsing level data).
///
/// The required further processing should happen in the [`ThunkContent`] implementation, which is
/// invoked by calling [`Thunk::process`]. Think of it as [`Cow`] with extra steps and potential new
/// allocations instead of cloning.
#[derive(Debug, Eq, Clone, Deserialize)]
#[serde(untagged)]
pub enum Thunk<'a, C: ThunkProcessor> {
    #[serde(skip)]
    Unprocessed(&'a str),
    Processed(C::Output<'a>)
}


impl<'a, 'b, P: ThunkProcessor> PartialEq<Thunk<'b, P>> for Thunk<'a, P>
    where
        P::Output<'a>: PartialEq<P::Output<'b>>
{
    fn eq(&self, other: &Thunk<'b, P>) -> bool {
        match (self, other) {
            (Thunk::Processed(o1), Thunk::Processed(o2)) => o1 == o2,
            (Thunk::Unprocessed(s1), Thunk::Unprocessed(s2)) => s1 == s2,
            _ => false
        }
    }
}

impl<'a, C: ThunkProcessor> Serialize for Thunk<'a, C>
where
    C::Output<'a>: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        match self {
            Thunk::Unprocessed(unprocessed) => C::from_unprocessed(unprocessed).map_err(S::Error::custom)?.serialize(serializer),
            Thunk::Processed(processed) => processed.serialize(serializer),
        }
    }
}

/// Trait describing how thunks should process their data
///
/// This trait provides the means to translate from and into RobTop's representation for thunked
/// data, while not being used in the (de)serialization into any other data format.
pub trait ThunkProcessor {
    type Error: std::error::Error;
    type Output<'a>;

    /// Takes some data from the [`Thunk::Unprocessed`] variant and processes it
    ///
    /// This function is *not* called automatically during deserialization from a RobTop data
    /// format.
    fn from_unprocessed<'a>(unprocessed: &'a str) -> Result<Self::Output<'a>, Self::Error>;

    /// Takes some processed thunk value and converts it into RobTop-representation
    fn as_unprocessed<'a, 'b>(processed: &'b Self::Output<'a>) -> Result<Cow<'b, str>, Self::Error>;
}

impl<'a, C: ThunkProcessor> Thunk<'a, C> {
    /// If this is a [`Thunk::Unprocessed`] variant, calls [`ThunkContent::from_unprocessed`] and
    /// returns [`Thunk::Processed`]. Simply returns `self` if this is a [`Thunk::Processed`]
    /// variant
    pub fn process(&mut self) -> Result<&C::Output<'a>, C::Error> {
        if let Thunk::Unprocessed(raw_data) = self {
            *self = Thunk::Processed(C::from_unprocessed(raw_data)?)
        }

        match self {
            Thunk::Processed(p) => Ok(p),
            _ => unreachable!(),
        }
    }

    pub fn as_unprocessed(&self) -> Result<Cow<str>, C::Error> {
        match self {
            Thunk::Unprocessed(unprocessed) => Ok(Cow::Borrowed(*unprocessed)),
            Thunk::Processed(content) => C::as_unprocessed(content)
        }
    }

    /// Returns the result of processing this [`Thunk`]
    pub fn into_processed(self) -> Result<C::Output<'a>, C::Error> {
        match self {
            Thunk::Unprocessed(unprocessed) => C::from_unprocessed(unprocessed),
            Thunk::Processed(p) => Ok(p),
        }
    }
}

/// Set of characters RobTop encodes when doing percent encoding
///
/// This is a subset of [`percent_encoding::NON_ALPHANUMERIC`], since that encodes too many
/// characters
pub const ROBTOP_SET: &AsciiSet = &CONTROLS
    .add(b' ')  // TODO: investigate if this is part of the set. Song links never contain spaces
    .add(b':')
    .add(b'/')
    .add(b'?')
    .add(b'~');

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct PercentDecoder;

impl ThunkProcessor for PercentDecoder {
    type Error = ProcessError;
    type Output<'a> = Cow<'a, str>;

    fn from_unprocessed<'a>(unprocessed: &'a str) -> Result<Self::Output<'a>, Self::Error> {
        percent_decode_str(unprocessed)
            .decode_utf8()
            .map_err(ProcessError::Utf8)
    }

    fn as_unprocessed<'a, 'b>(processed: &'b Self::Output<'a>) -> Result<Cow<'b, str>, Self::Error> {
        Ok(utf8_percent_encode(processed.as_ref(), ROBTOP_SET).into())
    }
}


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Base64Decoder;

impl ThunkProcessor for Base64Decoder {
    type Error = ProcessError;
    type Output<'a> = Cow<'a, str>;

    fn from_unprocessed<'a>(unprocessed: &'a str) -> Result<Self::Output<'a>, Self::Error> {
        let vec = base64::decode_config(unprocessed, URL_SAFE).map_err(ProcessError::Base64)?;
        let string = String::from_utf8(vec).map_err(ProcessError::FromUtf8)?;

        Ok(Cow::Owned(string))
    }

    fn as_unprocessed<'a, 'b>(processed: &'b Self::Output<'a>) -> Result<Cow<'b, str>, Self::Error> {
        Ok(Cow::Owned(base64::encode_config(&**processed, URL_SAFE)))
    }
}