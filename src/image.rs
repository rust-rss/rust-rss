// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::{BufRead, Write};

use quick_xml::Error as XmlError;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::events::attributes::Attributes;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::Error;
use crate::toxml::{ToXml, WriterExt};
use crate::util::element_text;

/// Represents an image in an RSS feed.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct Image {
    /// The URL of the image.
    url: String,
    /// A description of the image. This is used in the HTML `alt` attribute.
    title: String,
    /// The URL that the image links to.
    link: String,
    /// The width of the image.
    width: Option<String>,
    /// The height of the image.
    height: Option<String>,
    /// The text for the HTML `title` attribute of the link formed around the image.
    description: Option<String>,
}

impl Image {
    /// Return the URL of this image.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_url("http://example.com/image.png");
    /// assert_eq!(image.url(), "http://example.com/image.png");
    /// ```
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Set the URL of this image.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_url("http://example.com/image.png");
    /// ```
    pub fn set_url<V>(&mut self, url: V)
    where
        V: Into<String>,
    {
        self.url = url.into();
    }

    /// Return the description of this image.
    ///
    /// This is used in the HTML `alt` attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_title("Example image");
    /// assert_eq!(image.title(), "Example image");
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Set the description of this image.
    ///
    /// This is used in the HTML `alt` attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_title("Example image");
    /// ```
    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<String>,
    {
        self.title = title.into();
    }

    /// Return the URL that this image links to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_link("http://example.com");
    /// assert_eq!(image.link(), "http://example.com");
    pub fn link(&self) -> &str {
        self.link.as_str()
    }

    /// Set the URL that this image links to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_link("http://example.com");
    pub fn set_link<V>(&mut self, link: V)
    where
        V: Into<String>,
    {
        self.link = link.into();
    }

    /// Return the width of this image.
    ///
    /// If the width is `None` the default value should be considered to be `80`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_width("80".to_string());
    /// assert_eq!(image.width(), Some("80"));
    pub fn width(&self) -> Option<&str> {
        self.width.as_ref().map(String::as_str)
    }

    /// Set the width of this image.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_width("80".to_string());
    pub fn set_width<V>(&mut self, width: V)
    where
        V: Into<Option<String>>,
    {
        self.width = width.into();
    }

    /// Return the height of this image.
    ///
    /// If the height is `None` the default value should be considered to be `31`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_height("31".to_string());
    /// assert_eq!(image.height(), Some("31"));
    /// ```
    pub fn height(&self) -> Option<&str> {
        self.height.as_ref().map(String::as_str)
    }

    /// Set the height of this image.
    ///
    /// If the height is `None` the default value should be considered to be `31`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_height("31".to_string());
    /// ```
    pub fn set_height<V>(&mut self, height: V)
    where
        V: Into<Option<String>>,
    {
        self.height = height.into();
    }

    /// Return the title for the link formed around this image.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_description("Example Title".to_string());
    /// assert_eq!(image.description(), Some("Example Title"));
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    /// Set the title for the link formed around this image.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Image;
    ///
    /// let mut image = Image::default();
    /// image.set_description("Example Title".to_string());
    /// ```
    pub fn set_description<V>(&mut self, description: V)
    where
        V: Into<Option<String>>,
    {
        self.description = description.into();
    }
}

impl Image {
    /// Builds an Image from source XML
    pub fn from_xml<R: BufRead>(reader: &mut Reader<R>, _: Attributes) -> Result<Self, Error> {
        let mut image = Image::default();
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => match element.name() {
                    b"url" => image.url = element_text(reader)?.unwrap_or_default(),
                    b"title" => image.title = element_text(reader)?.unwrap_or_default(),
                    b"link" => image.link = element_text(reader)?.unwrap_or_default(),
                    b"width" => image.width = element_text(reader)?,
                    b"height" => image.height = element_text(reader)?,
                    b"description" => image.description = element_text(reader)?,
                    n => reader.read_to_end(n, &mut Vec::new())?,
                },
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }

            buf.clear();
        }

        Ok(image)
    }
}

impl ToXml for Image {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"image";

        writer.write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;

        writer.write_text_element(b"url", &self.url)?;
        writer.write_text_element(b"title", &self.title)?;
        writer.write_text_element(b"link", &self.link)?;

        if let Some(width) = self.width.as_ref() {
            writer.write_text_element(b"width", width)?;
        }

        if let Some(height) = self.height.as_ref() {
            writer.write_text_element(b"height", height)?;
        }

        if let Some(description) = self.description.as_ref() {
            writer.write_text_element(b"description", description)?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}
