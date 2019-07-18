
mod file_reader;
mod ftp_reader;
mod s3_reader;

// use crate::target_configuration::TargetConfiguration;

// use ftp::FtpError;

// use std::fs::File;
// use std::io::{BufReader, Cursor, Error, ErrorKind, Read};
// use std::path::Path;
// use tokio_io::AsyncRead;
// use tokio::prelude::*;

pub use file_reader::FileReader;
pub use ftp_reader::FtpReader;
pub use s3_reader::S3Reader;

#[test]
fn tranfer_ftp() {
  use crate::writer::FileStreamWriter;
  use crate::writer::StreamWriter;
  use std::env;

  let ftp_filename = env::var("FTP_FILENAME").expect("not variable FTP_FILENAME found in environment");
  let ftp_hostname = env::var("FTP_HOSTNAME").expect("not variable FTP_HOSTNAME found in environment");
  let ftp_username = env::var("FTP_USERNAME").expect("not variable FTP_USERNAME found in environment");
  let ftp_password = env::var("FTP_PASSWORD").expect("not variable FTP_PASSWORD found in environment");
  let ftp_prefix = env::var("FTP_PREFIX").expect("not variable FTP_PREFIX found in environment");

  let src_conf = TargetConfiguration::new_ftp(
    &ftp_hostname,
    &ftp_username,
    &ftp_password,
    &ftp_prefix,
    &ftp_filename
  );

  let mut reader = FtpReader::new(src_conf);
  let dst_conf = TargetConfiguration::new_file("/tmp/tranfer_test_ftp.raw");
  let mut writer = FileStreamWriter::new(dst_conf);
  writer.open().unwrap();
  reader.process_copy(move |stream| writer.write_stream(stream)).unwrap();
}

#[test]
fn tranfer_s3() {
  use crate::writer::FileStreamWriter;
  use crate::writer::StreamWriter;
  use std::env;
  use rusoto_core::region::Region;

  let access_key = env::var("AWS_ACCESS_KEY").expect("not variable AWS_ACCESS_KEY found in environment");
  let secret_key = env::var("AWS_SECRET_KEY").expect("not variable AWS_SECRET_KEY found in environment");
  let bucket = env::var("AWS_BUCKET").expect("not variable AWS_BUCKET found in environment");
  let filename = env::var("AWS_FILENAME").expect("not variable AWS_FILENAME found in environment");

  let src_conf = TargetConfiguration::new_s3(
    &access_key,
    &secret_key,
    Region::EuCentral1,
    &bucket,
    &filename
  );

  let mut reader = S3Reader::new(src_conf);
  let dst_conf = TargetConfiguration::new_file("/tmp/transfer_test_s3.raw");
  let mut writer = FileStreamWriter::new(dst_conf);
  writer.open().unwrap();
  reader.process_copy(move |stream| writer.write_stream(stream)).unwrap();
}
