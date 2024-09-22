use serde_json::json;
use std::io::{BufRead, BufReader, Cursor, Error, Read, Write};

static CONTENT_LENGTH: &str = "Content-Length: ";

pub fn marshall<R, W>(reader: &mut R, logger: &mut W) -> Option<serde_json::Value>
where
    R: BufRead,
    W: Write,
{
    let mut buf = String::new();
    let mut garbage_buf = String::new();
    reader.read_line(&mut buf);
    writeln!(logger, ">>> [request header] {}", buf);
    if buf.contains(CONTENT_LENGTH) {
        let x = buf.strip_prefix(CONTENT_LENGTH).unwrap().trim();
        writeln!(logger, ">>> [content length] {}", x);
        match x.parse::<usize>() {
            Ok(length) => {
                // skip the next line
                reader.read_line(&mut garbage_buf);

                let mut message_buf: Vec<u8> = vec![0; length];
                reader.read_exact(&mut message_buf);
                let body = String::from_utf8_lossy(&message_buf);
                writeln!(logger, ">>> [request body] {}", body);
                match serde_json::from_str(&body) {
                    Ok(res) => {
                        match res {
                            Some(json) => {
                                eprintln!("yu message: {:?}", json);
                                writeln!(logger, ">>> [parsed message] {:?}", json);
                                json
                            }
                            None => None
                        }
                    }
                    Err(e) => {
                        eprintln!("could not parse json");
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("could not parse content length: {}", e);
                None
            }
        }
    } else {
        None
    }
}

pub fn unmarshall<W>(writer: &mut W) -> Result<(), Error>
where
    W: Write,
{
    writer.write("hello world!".as_bytes());
    Ok(())
}


#[test]
fn test_parse_message() {
    let message = format!("Content-Length: 18\r\n\r\n{}", r#"{"name": "moamen"}"#);
    let cursor = Cursor::new(message);
    let mut reader = BufReader::new(cursor);
    let res = marshall(&mut reader);
    assert_eq!(Some(json!({"name": "moamen"})), res);
}


#[test]
fn test_parse_message_invalid_length() {
    let message = format!("Content-Length: 2\r\n\r\n{}", r#"{"name": "moamen"}"#);
    let cursor = Cursor::new(message);
    let mut reader = BufReader::new(cursor);
    let res = marshall(&mut reader);
    assert_eq!(None, res);
}

#[test]
fn test_parse_message_wrong_format() {
    let message = format!("Content_Length: 18\r\n\r\n{}", r#"{"name": "moamen"}"#);
    let cursor = Cursor::new(message);
    let mut reader = BufReader::new(cursor);
    let res = marshall(&mut reader);
    assert_eq!(None, res);
}
