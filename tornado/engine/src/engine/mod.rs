use crate::dispatcher::{DispatcherActor, ProcessedEventMessage};
use actix::prelude::*;
use log::*;
use std::sync::Arc;
use tornado_common_api;
use tornado_engine_api::api::handler::ProcessType;
use tornado_engine_matcher::model::ProcessedEvent;
use tornado_engine_matcher::{error, matcher};

pub struct EventMessageWithReply {
    pub event: tornado_common_api::Event,
    pub process_type: ProcessType,
}

impl Message for EventMessageWithReply {
    type Result = Result<ProcessedEvent, error::MatcherError>;
}

pub struct EventMessage {
    pub event: tornado_common_api::Event,
}

impl Message for EventMessage {
    type Result = Result<(), error::MatcherError>;
}

pub struct MatcherActor {
    pub dispatcher_addr: Addr<DispatcherActor>,
    pub matcher: Arc<matcher::Matcher>,
}

impl Actor for MatcherActor {
    type Context = SyncContext<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("MatcherActor started.");
    }
}

impl Handler<EventMessage> for MatcherActor {
    type Result = Result<(), error::MatcherError>;

    fn handle(&mut self, msg: EventMessage, _: &mut SyncContext<Self>) -> Self::Result {
        trace!("MatcherActor - received new EventMessage [{:?}]", &msg.event);

        let processed_event = self.matcher.process(msg.event);
        self.dispatcher_addr.do_send(ProcessedEventMessage { event: processed_event });
        Ok(())
    }
}

impl Handler<EventMessageWithReply> for MatcherActor {
    type Result = Result<ProcessedEvent, error::MatcherError>;

    fn handle(&mut self, msg: EventMessageWithReply, _: &mut SyncContext<Self>) -> Self::Result {
        trace!("MatcherActor - received new EventMessageWithReply [{:?}]", &msg.event);

        let processed_event = self.matcher.process(msg.event);

        match msg.process_type {
            ProcessType::Full => self
                .dispatcher_addr
                .do_send(ProcessedEventMessage { event: processed_event.clone() }),
            ProcessType::SkipActions => {}
        }

        Ok(processed_event)
    }
}
