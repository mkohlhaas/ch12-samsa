//! Samsa - A complete publish/subscribe microservice (Chapter 12)
//!
//! This is the culminating implementation that demonstrates all patterns from Chapters 9-11
//! plus Chapter 12's unique Rust features:
//!
//! - Result and Option patterns for robust error handling
//! - Block expressions for elegant initialization
//! - RAII patterns for automatic resource management
//! - Concise, expressive patterns leveraging Rust's unique features
//!
//! This complete service is production-ready and fully functional.

// Core error handling - unified across all modules
pub use error::{Result, SamsaError};

pub use broker::Broker;
pub use consumer::Consumer;
pub use message::{Event, Message};
pub use producer::Producer;

// Chapter 10 additions - Type System Patterns
pub use sealed::{JsonSchema, MessageHandler, MessageSchema, TextSchema, TypedMessage};
pub use types::{ConsumerId, MessageId, TopicId};
pub use typestate_consumer::{
    ConnectedConsumer, ConnectionInfo, Consumer as TypedConsumer, DisconnectedConsumer,
    PausedConsumer, SubscribedConsumer,
};

// Chapter 11 additions - Functional Programming Patterns
pub use closures::{EventBus, MessageFilter, MessageRouter, Pipeline, SystemEvent};
pub use pattern_matching::{
    ConfigValue, ConnectionEvent, ConnectionState, DatabaseConfig, MessageContent, Priority,
    ProcessingResult, RichMessage,
};
pub use pipeline::{
    AdvancedPipeline, EventTransformation, MessagePipeline, SubscriptionEvent,
    SubscriptionProcessing, SubscriptionStats,
};
pub use type_classes::{
    Activatable, Cancellable, MessageDeliverable, Subscription, SubscriptionManager, Suspendable,
};

// Chapter 12 additions - Unique Rust Features
pub use config::{BrokerConfig, BrokerConfigBuilder, SamsaConfig};
pub use resources::{ConnectionGuard, ConnectionPool, TransactionGuard};
pub use service::{BrokerService, ServiceManager};

mod broker;
mod consumer;
mod error;
mod message;
mod producer;
mod storage;

// Chapter 10 modules
pub mod sealed;
pub mod types;
pub mod typestate_consumer;

// Chapter 11 modules - Functional Programming
pub mod closures;
pub mod pattern_matching;
pub mod pipeline;
pub mod type_classes;

// Chapter 12 modules - Unique Rust Features
pub mod config;
pub mod resources;
pub mod service;
