#![deny(rust_2018_idioms, warnings)]
#![allow(
)]

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pem(pub Vec<u8>);

impl<'de> serde::Deserialize<'de> for Pem {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = Pem;

			fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(formatter, "a base64-encoded string")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
				Ok(Pem(v.as_bytes().to_owned()))
			}

			fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
				Ok(Pem(v.into_bytes()))
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}

impl serde::Serialize for Pem {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer {
		let s = std::str::from_utf8(&self.0).map_err(serde::ser::Error::custom)?;
		s.serialize(serializer)
	}
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Error {
	pub message: std::borrow::Cow<'static, str>,
}

pub mod create_cert {
	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	pub struct Request {
		#[serde(rename = "certId")]
		pub cert_id: String,

		pub csr: crate::Pem,

		pub issuer: Option<Issuer>,
	}

	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	pub struct Issuer {
		#[serde(rename = "certId")]
		pub cert_id: String,

		#[serde(rename = "privateKeyHandle")]
		pub private_key_handle: ks_common::KeyHandle,
	}

	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	pub struct Response {
		pub pem: crate::Pem,
	}
}

pub mod get_cert {
	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	pub struct Response {
		pub pem: crate::Pem,
	}
}

pub mod import_cert {
	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	pub struct Request {
		pub pem: crate::Pem,
	}

	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	pub struct Response {
		pub pem: crate::Pem,
	}
}
