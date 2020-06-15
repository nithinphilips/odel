use async_trait::async_trait;
use std::io::{BufRead, Cursor};
use yaserde::{YaDeserialize, YaSerialize};
use crate::soap;
use regex::Regex;
use reqwest::Client;
use thiserror::Error;
use hyper::StatusCode;
use crate::tririga::tririga::{GetHttpSession, GetHttpSessionResponse};
use crate::tririga::dto::HttpSession;
use crate::tririga::transport::Error::SoapError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error occurred while creating a SOAP session: `{0}`")]
    Session(String),
    #[error("Error occurred while serializing data: `{0}`")]
    Serialization(String),
    #[error("Error occurred while deserializing data: `{0}`")]
    Deserialization(String),
    #[error("Error occurred while communicating with remote SOAP service: `{0}`")]
    Transport(anyhow::Error),
    #[error("HTTP client error: `{0}`")]
    HttpClient(StatusCode),
    #[error("HTTP server error: `{0}`")]
    HttpServer(StatusCode),
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    SoapError(#[from] soap::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

const TRIRIGA_AUTH_OBJECTID: &str = "1000";
#[allow(dead_code)]
const TRIRIGA_AUTH_LOGIN_ACTIONID: &str = "1005";
const TRIRIGA_AUTH_FORCELOGIN_ACTIONID: &str = "1006";

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::Transport(anyhow!(e.to_string()))
    }
}

#[async_trait]
pub trait Transport {
    async fn request(&self, message: &str) -> Result<String, Error>;
    async fn login_soap(&self) -> Result<HttpSession, Error>;
    async fn login_web(&self) -> Result<(), Error>;
}

pub struct HttpTransport
{
    pub client: Client,
    pub url: String,
    pub endpoint: String,
    pub username: String,
    pub password: String
}

impl HttpTransport {

}

#[async_trait]
impl Transport for HttpTransport {
    async fn request(&self, message: &str) -> Result<String, Error> {
        request_inner(self, message, false).await
    }

    async fn login_soap(&self) -> Result<HttpSession, Error> {
        let request_obj = GetHttpSession {};
        let response_obj: GetHttpSessionResponse = request_with_auth(self, &request_obj).await?;
        Ok(response_obj.out.ok_or(Error::Session("Invalid response to login request".to_string()))?)
    }

    async fn login_web(&self) -> Result<(), Error>
    {
        let login_params = vec![
            ("USERNAME", self.username.as_str()),
            ("PASSWORD", self.password.as_str()),
            ("objectId", TRIRIGA_AUTH_OBJECTID),
            ("actionId", TRIRIGA_AUTH_FORCELOGIN_ACTIONID)
        ];

        debug!("Create session");
        self.client.get(&format!("{}/login", self.url)).send().await?;

        debug!("Login");
        self.client.post(&format!("{}/Authenticate.srv", self.url))
            .form(&login_params)
            .send().await?;

        Ok(())
    }
}

async fn request_inner(transport: &HttpTransport, message: &str, include_auth: bool) ->
                                                                                   Result<String, Error> {
    let soap_message = soap::soap(&message, &None).unwrap();

    debug!("Request SOAP: {}", soap_message);

    let mut request = transport.client.post(&transport.endpoint)
        .header(hyper::header::CONTENT_TYPE, "application/soap+xml; charset=utf-8;")
        .body(soap_message);

    if include_auth {
        request = request
            .basic_auth(&transport.username, Some(&transport.password))
            .header("Username", &transport.username)
            .header("Password", &transport.password)
    }

    debug!("Request: {:?}", request);

    let response = request.send().await?;

    debug!("Response: {}", response.status());
    debug!("Headers: {:#?}\n", response.headers());

    if response.status().is_client_error() {
        let e = Err(Error::HttpClient(response.status()));
        let response_text = Vec::from(response.text().await?);
        trace!("Message: {:?}", std::str::from_utf8(&response_text).unwrap());
        return e;
    }
    // else if response.status().is_server_error() {
    //     let e = Err(Error::HttpServer(response.status()));
    //     let response_text = Vec::from(response.text().await?);
    //     trace!("Message: {:?}", std::str::from_utf8(&response_text).unwrap());
    //     return e;
    // }

    let mut nodes: Vec<Vec<u8>> = Vec::new();
    let headers = response.headers().clone();
    let response_text = Vec::from(response.text().await?);
    // trace!("Body: {:?}", std::str::from_utf8(&response_text).unwrap());
    let mut response_text = Cursor::new(response_text);
    inner(&mut response_text, &headers, &mut nodes, false)?;

    let first_body = nodes.first().ok_or( Error::Deserialization("Could not find multi-part content".to_string()))?;
    let str = std::str::from_utf8(first_body)?;
    //debug!("{:?}", str);


    let unsoap_message = soap::unsoap(&str)?;

    Ok(unsoap_message)
}

fn inner<R: BufRead>(
    reader: &mut R,
    headers: &hyper::header::HeaderMap,
    nodes: &mut Vec<Vec<u8>>,
    always_use_files: bool)
    -> Result<(), Error>
{
    use buf_read_ext::BufReadExt;

    let mut buf: Vec<u8> = Vec::new();

    let boundary = get_multipart_boundary(headers);

    // Read past the initial boundary
    let (_, found) = reader.stream_until_token(&boundary, &mut buf)?;
    if ! found { return Err(Error::Transport(anyhow!("Token not found"))); }

    // Define the boundary, including the line terminator preceding it.
    // Use their first line terminator to determine whether to use CRLF or LF.
    let (lt, ltlt, lt_boundary) = {
        let peeker = reader.fill_buf()?;
        if peeker.len() > 1 && &peeker[..2]==b"\r\n" {
            let mut output = Vec::with_capacity(2 + boundary.len());
            output.push(b'\r');
            output.push(b'\n');
            output.extend(boundary.clone());
            (vec![b'\r', b'\n'], vec![b'\r', b'\n', b'\r', b'\n'], output)
        }
        else if peeker.len() > 0 && peeker[0]==b'\n' {
            let mut output = Vec::with_capacity(1 + boundary.len());
            output.push(b'\n');
            output.extend(boundary.clone());
            (vec![b'\n'], vec![b'\n', b'\n'], output)
        }
        else {
            return Err(Error::Transport(anyhow!("NoCrLfAfterBoundary")));
        }
    };

    loop {
        // If the next two lookahead characters are '--', parsing is finished.
        {
            let peeker = reader.fill_buf()?;
            if peeker.len() >= 2 && &peeker[..2] == b"--" {
                return Ok(());
            }
        }

        // Read the line terminator after the boundary
        let (_, found) = reader.stream_until_token(&lt, &mut buf)?;
        if !found { return Err("NoCrLfAfterBoundary".into()); }

        // Read the headers (which end in 2 line terminators)
        buf.truncate(0); // start fresh
        let (_, found) = reader.stream_until_token(&ltlt, &mut buf)?;
        if !found { return Err("EofInPartHeaders".into()); }

        // Keep the 2 line terminators as httparse will expect it
        buf.extend(ltlt.iter().cloned());

        // Parse the headers
        // let part_headers = {
        //     let mut header_memory = [httparse::EMPTY_HEADER; 4];
        //     match httparse::parse_headers(&buf, &mut header_memory) {
        //         Ok(httparse::Status::Complete((_, raw_headers))) => {
        //             raw_headers
        //         },
        //         Ok(httparse::Status::Partial) => panic!("Error"),
        //         Err(err) => panic!("Error")
        //     }
        // };

        buf.truncate(0); // start fresh
        let (_, found) = reader.stream_until_token(&lt_boundary, &mut buf)?;
        if !found { return Err("EofInPart".into()); }

        nodes.push( buf.clone());
    }
}

pub fn get_multipart_boundary(headers: &hyper::header::HeaderMap) -> Vec<u8> {

    let ct = headers.get(hyper::header::CONTENT_TYPE).unwrap().to_str().unwrap();


    let re = Regex::new(r#"boundary="(?P<boundary>[^"]*)""#)
        .unwrap();

    let caps = re.captures(&ct).unwrap();

    let mut result: Vec<u8> = Vec::from("--");
    result.extend_from_slice(&caps["boundary"].as_bytes());
    result
}

pub trait Validate {
}

pub async fn request<T: Transport, R: YaSerialize, S: YaDeserialize>(
    transport: &T,
    request: &R,
) -> Result<S, Error> {
    let ser = |obj: &R| yaserde::ser::to_string(obj).map_err(Error::Serialization);

    let req = yaserde::ser::to_string(request).map_err(Error::Serialization);

    let de = |s: &str| yaserde::de::from_str(s).map_err(Error::Deserialization);

    de(&transport
        .request(&crop_xml_declaration(&ser(&request)?))
        .await?)
}

pub async fn request_with_auth<R: YaSerialize, S: YaDeserialize>(
    transport: &HttpTransport,
    request: &R,
) -> Result<S, Error> {
    let ser = |obj: &R| yaserde::ser::to_string(obj).map_err(Error::Serialization);

    let req = yaserde::ser::to_string(request).map_err(Error::Serialization);

    let de = |s: &str| yaserde::de::from_str(s).map_err(Error::Deserialization);

    let res = request_inner(&transport,
                            &crop_xml_declaration(&ser(&request)?),
                            true).await?;
    de(&res)
}

fn crop_xml_declaration(xml: &str) -> String {
    xml.split("?>").skip(1).collect()
}

#[test]
fn test_crop_xml_declaration() {
    assert_eq!(
        crop_xml_declaration(r#"<?xml version="1.0" encoding="utf-8"?><element />"#),
        "<element />"
    );
}


