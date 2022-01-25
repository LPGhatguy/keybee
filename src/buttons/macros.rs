#[macro_export]
macro_rules! wrapper_enum {
    ( $enum_name:ident {
        $( $variant:ident($inner:ident, $inner_name:ident), )*
    } ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $enum_name {
            $( $variant($inner), )*
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( Self::$variant(inner) => inner.fmt(f), )*
                }
            }
        }

        impl std::str::FromStr for $enum_name {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut pieces = s.splitn(2, '/');
                let device = pieces.next().unwrap();

                match device {
                    $( stringify!($inner_name) => Ok(Self::$variant(s.parse()?)), )*
                    _ => anyhow::bail!("invalid keybee device '{}'", device),
                }
            }
        }

        impl serde::Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let as_str = String::deserialize(deserializer)?;
                as_str.parse().map_err(|err| serde::de::Error::custom(format!("{:?}", err)))
            }
        }
    };
}

#[macro_export]
macro_rules! keyboard {
    ( $( $variant:ident($name:ident), )* ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum KeyboardKey {
            $( $variant, )*
        }

        impl std::str::FromStr for KeyboardKey {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( concat!("keyboard/", stringify!($name)) => Ok(Self::$variant), )*
                    _ => anyhow::bail!("unknown keyboard key '{}'", s),
                }
            }
        }

        impl std::fmt::Display for KeyboardKey {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( Self::$variant => write!(f, "keyboard/{}", stringify!($name)), )*
                }
            }
        }

        #[cfg(feature = "winit")]
        impl From<KeyboardKey> for winit::event::VirtualKeyCode {
            fn from(value: KeyboardKey) -> Self {
                match value {
                    $( KeyboardKey::$variant => Self::$variant, )*
                }
            }
        }

        #[cfg(feature = "winit")]
        impl TryFrom<winit::event::VirtualKeyCode> for KeyboardKey {
            type Error = anyhow::Error;

            fn try_from(value: winit::event::VirtualKeyCode) -> Result<Self, Self::Error> {
                match value {
                    $( winit::event::VirtualKeyCode::$variant => Ok(Self::$variant), )*
                    _ => anyhow::bail!("{:?} is not supported by keybee", value)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_device {
    (
        $input:ident($input_name:ident)
        $(
            $enum_variant:ident($enum:ident {
                $( $variant:ident($name:ident), )*
            })
        )*
    ) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum $enum {
                $( $variant, )*
            }

            impl std::fmt::Display for $enum {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let button = match self {
                        $( Self::$variant => stringify!($name), )*
                    };

                    write!(f, "{}/{}", stringify!($input_name), button)
                }
            }

            impl std::str::FromStr for $enum {
                type Err = anyhow::Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        $( stringify!($name) => Ok(Self::$variant), )*
                        _ => anyhow::bail!("unknown {} input '{}'", stringify!(input_name), s),
                    }
                }
            }
        )*

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $input {
            $( $enum_variant($enum), )*
        }

        impl std::fmt::Display for $input {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( Self::$enum_variant(inner) => inner.fmt(f), )*
                }
            }
        }

        impl std::str::FromStr for $input {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $( concat!(stringify!($input_name), "/", stringify!($name))
                            => Ok(Self::$enum_variant($enum::$variant)), )*
                    )*
                    _ => anyhow::bail!("unknown {} input '{}'", stringify!($input_name), s),
                }
            }
        }
    };
}
