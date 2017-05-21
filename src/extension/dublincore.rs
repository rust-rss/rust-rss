// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use extension::Extension;
use extension::remove_extension_values;
use quick_xml::XmlWriter;
use quick_xml::error::Error as XmlError;
use std::collections::HashMap;
use toxml::{ToXml, XmlWriterExt};

/// The Dublin Core XML namespace.
pub static NAMESPACE: &'static str = "http://purl.org/dc/elements/1.1/";

/// A Dublin Core element extension.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DublinCoreExtension {
    /// An entity responsible for making contributions to the resource.
    contributors: Vec<String>,
    /// The spatial or temporal topic of the resource, the spatial applicability of the resource,
    /// or the jurisdiction under which the resource is relevant.
    coverages: Vec<String>,
    /// An entity primarily responsible for making the resource.
    creators: Vec<String>,
    /// A point or period of time associated with an event in the lifecycle of the resource.
    dates: Vec<String>,
    /// An account of the resource.
    descriptions: Vec<String>,
    /// The file format, physical medium, or dimensions of the resource.
    formats: Vec<String>,
    /// An unambiguous reference to the resource within a given context.
    identifiers: Vec<String>,
    /// A language of the resource.
    languages: Vec<String>,
    /// An entity responsible for making the resource available.
    publishers: Vec<String>,
    /// A related resource.
    relations: Vec<String>,
    /// Information about rights held in and over the resource.
    rights: Vec<String>,
    /// A related resource from which the described resource is derived.
    sources: Vec<String>,
    /// The topic of the resource.
    subjects: Vec<String>,
    /// A name given to the resource.
    titles: Vec<String>,
    /// The nature or genre of the resource.
    resource_types: Vec<String>,
}

impl DublinCoreExtension {
    /// Get the contributors that exists under `DublinCoreExtension`.
    pub fn contributors(&self) -> &[String] {
        &self.contributors
    }

    /// Get the coverages that exists under `DublinCoreExtension`.
    pub fn coverages(&self) -> &[String] {
        &self.coverages
    }

    /// Get the creators that exists under `DublinCoreExtension`.
    pub fn creators(&self) -> &[String] {
        &self.creators
    }

    /// Get the dates that exists under `DublinCoreExtension`.
    pub fn dates(&self) -> &[String] {
        &self.dates
    }

    /// Get the descriptions that exists under `DublinCoreExtension`.
    pub fn descriptions(&self) -> &[String] {
        &self.descriptions
    }

    /// Get the formats that exists under `DublinCoreExtension`.
    pub fn formats(&self) -> &[String] {
        &self.formats
    }

    /// Get the identifiers that exists under `DublinCoreExtension`.
    pub fn identifiers(&self) -> &[String] {
        &self.identifiers
    }

    /// Get the languages that exists under `DublinCoreExtension`.
    pub fn languages(&self) -> &[String] {
        &self.languages
    }

    /// Get the publishers that exists under `DublinCoreExtension`.
    pub fn publishers(&self) -> &[String] {
        &self.publishers
    }

    /// Get the relations that exists under `DublinCoreExtension`.
    pub fn relations(&self) -> &[String] {
        &self.relations
    }

    /// Get the rights that exists under `DublinCoreExtension`.
    pub fn rights(&self) -> &[String] {
        &self.rights
    }

    /// Get the sources that exists under `DublinCoreExtension`.
    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    /// Get the subjects that exists under `DublinCoreExtension`.
    pub fn subjects(&self) -> &[String] {
        &self.subjects
    }

    /// Get the titles that exists under `DublinCoreExtension`.
    pub fn titles(&self) -> &[String] {
        &self.titles
    }

    /// Get the resource_types that exists under `DublinCoreExtension`.
    pub fn resource_types(&self) -> &[String] {
        &self.resource_types
    }
}

impl DublinCoreExtension {
    /// Creates a DublinCoreExtension using the specified hashmap.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Self {
        let mut ext = DublinCoreExtension::default();
        ext.contributors = remove_extension_values(&mut map, "contributor").unwrap_or_default();
        ext.coverages = remove_extension_values(&mut map, "coverage").unwrap_or_default();
        ext.creators = remove_extension_values(&mut map, "creator").unwrap_or_default();
        ext.dates = remove_extension_values(&mut map, "date").unwrap_or_default();
        ext.descriptions = remove_extension_values(&mut map, "description").unwrap_or_default();
        ext.formats = remove_extension_values(&mut map, "format").unwrap_or_default();
        ext.identifiers = remove_extension_values(&mut map, "identifier").unwrap_or_default();
        ext.languages = remove_extension_values(&mut map, "language").unwrap_or_default();
        ext.publishers = remove_extension_values(&mut map, "publisher").unwrap_or_default();
        ext.relations = remove_extension_values(&mut map, "relation").unwrap_or_default();
        ext.rights = remove_extension_values(&mut map, "rights").unwrap_or_default();
        ext.sources = remove_extension_values(&mut map, "source").unwrap_or_default();
        ext.subjects = remove_extension_values(&mut map, "subject").unwrap_or_default();
        ext.titles = remove_extension_values(&mut map, "title").unwrap_or_default();
        ext.resource_types = remove_extension_values(&mut map, "type").unwrap_or_default();
        ext
    }
}

impl ToXml for DublinCoreExtension {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        writer
            .write_text_elements(b"dc:contributor", &self.contributors)?;
        writer
            .write_text_elements(b"dc:coverage", &self.coverages)?;
        writer
            .write_text_elements(b"dc:creator", &self.creators)?;
        writer.write_text_elements(b"dc:date", &self.dates)?;
        writer
            .write_text_elements(b"dc:description", &self.descriptions)?;
        writer.write_text_elements(b"dc:format", &self.formats)?;
        writer
            .write_text_elements(b"dc:identifier", &self.identifiers)?;
        writer
            .write_text_elements(b"dc:language", &self.languages)?;
        writer
            .write_text_elements(b"dc:publisher", &self.publishers)?;
        writer
            .write_text_elements(b"dc:relation", &self.relations)?;
        writer.write_text_elements(b"dc:rights", &self.rights)?;
        writer.write_text_elements(b"dc:source", &self.sources)?;
        writer
            .write_text_elements(b"dc:subject", &self.subjects)?;
        writer.write_text_elements(b"dc:title", &self.titles)?;
        writer.write_text_elements(b"dc:type", &self.resource_types)
    }
}

/// This `DublinCoreExtensionBuilder` struct creates the `DublinCoreExtension`.
#[derive(Debug, Default, Clone)]
pub struct DublinCoreExtensionBuilder {
    /// An entity responsible for making contributions to the resource.
    contributors: Vec<String>,
    /// The spatial or temporal topic of the resource, the spatial applicability of the resource,
    /// or the jurisdiction under which the resource is relevant.
    coverages: Vec<String>,
    /// An entity primarily responsible for making the resource.
    creators: Vec<String>,
    /// A point or period of time associated with an event in the lifecycle of the resource.
    dates: Vec<String>,
    /// An account of the resource.
    descriptions: Vec<String>,
    /// The file format, physical medium, or dimensions of the resource.
    formats: Vec<String>,
    /// An unambiguous reference to the resource within a given context.
    identifiers: Vec<String>,
    /// A language of the resource.
    languages: Vec<String>,
    /// An entity responsible for making the resource available.
    publishers: Vec<String>,
    /// A related resource.
    relations: Vec<String>,
    /// Information about rights held in and over the resource.
    rights: Vec<String>,
    /// A related resource from which the described resource is derived.
    sources: Vec<String>,
    /// The topic of the resource.
    subjects: Vec<String>,
    /// A name given to the resource.
    titles: Vec<String>,
    /// The nature or genre of the resource.
    resource_types: Vec<String>,
}

impl DublinCoreExtensionBuilder {
    /// Construct a new `DublinCoreExtensionBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::dublincore::DublinCoreExtensionBuilder;
    ///
    /// let dublin_builder = DublinCoreExtensionBuilder::new();
    /// ```
    pub fn new() -> DublinCoreExtensionBuilder {
        DublinCoreExtensionBuilder::default()
    }

    /// Set the contributors that exists under `DublinCoreExtension`.
    pub fn contributors(mut self, contributors: Vec<String>) -> DublinCoreExtensionBuilder {
        self.contributors = contributors;
        self
    }

    /// Set the coverages that exists under `DublinCoreExtension`.
    pub fn coverages(mut self, coverages: Vec<String>) -> DublinCoreExtensionBuilder {
        self.coverages = coverages;
        self
    }

    /// Set the creators that exists under `DublinCoreExtension`.
    pub fn creators(mut self, creators: Vec<String>) -> DublinCoreExtensionBuilder {
        self.creators = creators;
        self
    }

    /// Set the dates that exists under `DublinCoreExtension`.
    pub fn dates(mut self, dates: Vec<String>) -> DublinCoreExtensionBuilder {
        self.dates = dates;
        self
    }

    /// Set the descriptions that exists under `DublinCoreExtension`.
    pub fn descriptions(mut self, descriptions: Vec<String>) -> DublinCoreExtensionBuilder {
        self.descriptions = descriptions;
        self
    }

    /// Set the formats that exists under `DublinCoreExtension`.
    pub fn formats(mut self, formats: Vec<String>) -> DublinCoreExtensionBuilder {
        self.formats = formats;
        self
    }

    /// Set the identifiers that exists under `DublinCoreExtension`.
    pub fn identifiers(mut self, identifiers: Vec<String>) -> DublinCoreExtensionBuilder {
        self.identifiers = identifiers;
        self
    }

    /// Set the languages that exists under `DublinCoreExtension`.
    pub fn languages(mut self, languages: Vec<String>) -> DublinCoreExtensionBuilder {
        self.languages = languages;
        self
    }

    /// Set the publishers that exists under `DublinCoreExtension`.
    pub fn publishers(mut self, publishers: Vec<String>) -> DublinCoreExtensionBuilder {
        self.publishers = publishers;
        self
    }

    /// Set the relations that exists under `DublinCoreExtension`.
    pub fn relations(mut self, relations: Vec<String>) -> DublinCoreExtensionBuilder {
        self.relations = relations;
        self
    }

    /// Set the rights that exists under `DublinCoreExtension`.
    pub fn rights(mut self, rights: Vec<String>) -> DublinCoreExtensionBuilder {
        self.rights = rights;
        self
    }

    /// Set the sources that exists under `DublinCoreExtension`.
    pub fn sources(mut self, sources: Vec<String>) -> DublinCoreExtensionBuilder {
        self.sources = sources;
        self
    }

    /// Set the subjects that exists under `DublinCoreExtension`.
    pub fn subjects(mut self, subjects: Vec<String>) -> DublinCoreExtensionBuilder {
        self.subjects = subjects;
        self
    }

    /// Set the titles that exists under `DublinCoreExtension`.
    pub fn titles(mut self, titles: Vec<String>) -> DublinCoreExtensionBuilder {
        self.titles = titles;
        self
    }

    /// Set the resource_types that exists under `DublinCoreExtension`.
    pub fn resource_types(mut self, resource_types: Vec<String>) -> DublinCoreExtensionBuilder {
        self.resource_types = resource_types;
        self
    }

    /// Construct the `DublinCoreExtension` from the `DublinCoreExtensionBuilder`.
    pub fn finalize(self) -> Result<DublinCoreExtension, Error> {
        Ok(DublinCoreExtension {
               contributors: self.contributors,
               coverages: self.coverages,
               creators: self.creators,
               dates: self.dates,
               descriptions: self.descriptions,
               formats: self.formats,
               identifiers: self.identifiers,
               languages: self.languages,
               publishers: self.publishers,
               relations: self.relations,
               rights: self.rights,
               sources: self.sources,
               subjects: self.subjects,
               titles: self.titles,
               resource_types: self.resource_types,
           })
    }
}
