#[macro_export]
macro_rules! define_exchange_error {
    ($name:ident, $default_msg:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub message: String,
            source: Option<std::sync::Arc<dyn std::error::Error + Send + Sync + 'static>>,
        }

        impl $name {
            pub const DEFAULT_MESSAGE: &'static str = $default_msg;

            pub fn new() -> Self {
                Self {
                    message: Self::DEFAULT_MESSAGE.to_string(),
                    source: None,
                }
            }

            pub fn with_message(msg: impl Into<String>) -> Self {
                Self {
                    message: msg.into(),
                    source: None,
                }
            }

            pub fn with_source<E>(cause: E) -> Self
            where
                E: std::error::Error + Send + Sync + 'static,
            {
                Self {
                    message: Self::DEFAULT_MESSAGE.to_string(),
                    source: Some(std::sync::Arc::new(cause)),
                }
            }

            pub fn with_message_and_source<E>(msg: impl Into<String>, cause: E) -> Self
            where
                E: std::error::Error + Send + Sync + 'static,
            {
                Self {
                    message: msg.into(),
                    source: Some(std::sync::Arc::new(cause)),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", stringify!($name), self.message)
            }
        }

        impl std::error::Error for $name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.source
                    .as_ref()
                    .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
            }
        }

        impl crate::error::ExchangeErrorDetail for $name {}

        impl From<$name> for crate::error::exchange_error::ExchangeError {
            fn from(err: $name) -> Self {
                crate::error::exchange_error::ExchangeError::Custom(Box::new(err))
            }
        }
    };
}
