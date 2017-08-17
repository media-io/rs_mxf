
use std::io::{Error, ErrorKind, Read};
use serializer::decoder::*;
use klv::ul::Ul;
use klv::ul::ul::*;

impl Decoder for Ul {
  fn deserialize<R: Read>(&mut self, stream: &mut R) -> Result<bool, Error> {
    let mut identifier_data = vec![0; 16];
    match stream.read(&mut identifier_data) {
      Ok(16) => {
      },
      Ok(0) => {
        return Ok(false);
      },
      Ok(_bad_length) => {
        return Err(Error::new(ErrorKind::UnexpectedEof, "unable to read 16 bytes for UL detection"));
      },
      Err(msg) => {
        return Err(msg);
      },
    };

    match match_ul(identifier_data) {
      Some(ul) => {
        *self = ul;
      },
      None => {},
    };
    return Ok(true);
  }
}
