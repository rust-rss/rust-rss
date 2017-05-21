// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use fromxml::FromXml;
use quick_xml::{Element, Event, XmlReader, XmlWriter};
use quick_xml::error::Error as XmlError;
use reqwest::Url;
use std::str::FromStr;
use toxml::ToXml;

/// A representation of the `<cloud>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cloud {
    /// The domain to register with.
    domain: String,
    /// The port to register with.
    port: String,
    /// The path to register with.
    path: String,
    /// The procedure to register with.
    register_procedure: String,
    /// The protocol to register with.
    protocol: String,
}

impl Cloud {
    /// Get the domain that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CloudBuilder, Cloud};
    ///
    /// let domain = "http://rpc.sys.com/";
    ///
    /// let cloud = CloudBuilder::new()
    ///     .domain(domain)
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(domain.to_string(), cloud.domain());
    /// ```
    pub fn domain(&self) -> &str {
        self.domain.as_str()
    }


    /// Get the port that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CloudBuilder, Cloud};
    ///
    /// let port: i64 = 80;
    ///
    /// let cloud = CloudBuilder::new()
    ///     .port(port)
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(port.to_string(), cloud.port());
    /// ```
    pub fn port(&self) -> &str {
        self.port.as_str()
    }


    /// Get the path that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CloudBuilder, Cloud};
    ///
    /// let path = "/RPC2";
    ///
    /// let cloud = CloudBuilder::new()
    ///     .path(path)
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(path, cloud.path());
    /// ```
    pub fn path(&self) -> &str {
        self.path.as_str()
    }


    /// Get the register procedure that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CloudBuilder, Cloud};
    ///
    /// let register_procedure = "pingMe";
    /// let cloud = CloudBuilder::new()
    ///     .register_procedure(register_procedure)
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(register_procedure, cloud.register_procedure());
    /// ```
    pub fn register_procedure(&self) -> &str {
        self.register_procedure.as_str()
    }


    /// Get the protocol that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CloudBuilder, Cloud};
    ///
    /// let protocol = "soap";
    ///
    /// let cloud = CloudBuilder::new()
    ///     .protocol(protocol)
    ///     .domain("http://rpc.sys.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(protocol, cloud.protocol());
    /// ```
    pub fn protocol(&self) -> &str {
        self.protocol.as_str()
    }
}

impl FromXml for Cloud {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut domain = None;
        let mut port = None;
        let mut path = None;
        let mut register_procedure = None;
        let mut protocol = None;

        for attr in element.attributes().with_checks(false).unescaped() {
            if let Ok(attr) = attr {
                match attr.0 {
                    b"domain" if domain.is_none() => {
                        domain = Some(String::from_utf8(attr.1.into_owned())?);
                    }
                    b"port" if port.is_none() => {
                        port = Some(String::from_utf8(attr.1.into_owned())?);
                    }
                    b"path" if path.is_none() => {
                        path = Some(String::from_utf8(attr.1.into_owned())?);
                    }
                    b"registerProcedure" if register_procedure.is_none() => {
                        register_procedure = Some(String::from_utf8(attr.1.into_owned())?);
                    }
                    b"protocol" if protocol.is_none() => {
                        protocol = Some(String::from_utf8(attr.1.into_owned())?);
                    }
                    _ => {}
                }
            }
        }

        skip_element!(reader);

        let domain = domain.unwrap_or_default();
        let port = port.unwrap_or_default();
        let path = path.unwrap_or_default();
        let register_procedure = register_procedure.unwrap_or_default();
        let protocol = protocol.unwrap_or_default();

        Ok((Cloud {
                domain: domain,
                port: port,
                path: path,
                register_procedure: register_procedure,
                protocol: protocol,
            },
            reader))

    }
}

impl ToXml for Cloud {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        let element = Element::new(b"cloud");

        writer
            .write(Event::Start({
                                    let mut element = element.clone();

                                    let attrs = &[(b"domain" as &[u8], &self.domain),
                                                  (b"port", &self.port),
                                                  (b"path", &self.path),
                                                  (b"registerProcedure", &self.register_procedure),
                                                  (b"protocol", &self.protocol)];
                                    element.extend_attributes(attrs.into_iter().map(|v| *v));

                                    element
                                }))?;

        writer.write(Event::End(element))
    }
}

/// This `CloudBuilder` struct creates the `Cloud`.
#[derive(Debug, Clone, Default)]
pub struct CloudBuilder {
    domain: String,
    port: i64,
    path: String,
    register_procedure: String,
    protocol: String,
}


impl CloudBuilder {
    /// Construct a new `CloudBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CloudBuilder;
    ///
    /// let cloud_builder = CloudBuilder::new();
    /// ```
    pub fn new() -> CloudBuilder {
        CloudBuilder::default()
    }


    /// Set the domain that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.domain("http://rpc.sys.com/");
    /// ```
    pub fn domain(mut self, domain: &str) -> CloudBuilder {
        self.domain = domain.to_string();
        self
    }


    /// Set the port that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.port(80);
    /// ```
    pub fn port(mut self, port: i64) -> CloudBuilder {

        self.port = port;
        self
    }


    /// Set the path that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.path("/RPC2");
    /// ```
    pub fn path(mut self, path: &str) -> CloudBuilder {
        self.path = path.to_string();
        self
    }


    /// Set the register procedure that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.register_procedure("pingMe");
    /// ```
    pub fn register_procedure(mut self, register_procedure: &str) -> CloudBuilder {
        self.register_procedure = register_procedure.to_string();
        self
    }


    /// Set the protocol that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.protocol("soap");
    /// ```
    pub fn protocol(mut self, protocol: &str) -> CloudBuilder {
        self.protocol = protocol.to_string();
        self
    }


    /// Validate the contents of `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CloudBuilder;
    ///
    /// let cloud = CloudBuilder::new()
    ///         .domain("http://rpc.sys.com/")
    ///         .port(80)
    ///         .path("/RPC2")
    ///         .register_procedure("pingMe")
    ///         .protocol("soap")
    ///         .validate().unwrap()
    ///         .finalize().unwrap();
    /// ```
    pub fn validate(self) -> Result<CloudBuilder, Error> {
        if self.port < 0 {
            return Err(Error::Validation("Cloud Port cannot be a negative value".to_string()));
        }

        Url::parse(self.domain.as_str())?;

        match CloudProtocol::from_str(self.protocol.as_str()) {
            Ok(_) => (),
            Err(err) => return Err(Error::Validation(err.to_string())),
        };

        Ok(self)
    }


    /// Construct the `Cloud` from the `CloudBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CloudBuilder;
    ///
    /// let cloud = CloudBuilder::new()
    ///         .domain("http://rpc.sys.com/")
    ///         .port(80)
    ///         .path("/RPC2")
    ///         .register_procedure("pingMe")
    ///         .protocol("soap")
    ///         .finalize();
    /// ```
    pub fn finalize(self) -> Result<Cloud, Error> {
        let port = self.port.to_string();

        Ok(Cloud {
               domain: self.domain,
               port: port,
               path: self.path,
               register_procedure: self.register_procedure,
               protocol: self.protocol,
           })
    }
}


/// Enumerations of protocols for `Cloud`.
#[derive(Clone, Debug)]
enum CloudProtocol {
    /// http-post
    HttpPost,

    /// xml-rpc
    XmlRpc,

    /// soap
    Soap,
}


impl FromStr for CloudProtocol {
    type Err = &'static str;

    // Convert `&str` to `CloudProtocol`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http-post" => Ok(CloudProtocol::HttpPost),
            "xml-rpc" => Ok(CloudProtocol::XmlRpc),
            "soap" => Ok(CloudProtocol::Soap),
            _ => Err("not a valid value"),
        }
    }
}
