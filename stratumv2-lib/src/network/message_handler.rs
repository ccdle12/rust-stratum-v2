// use crate::{common, error::Result, frame::Message, mining};

// // NOTE: SetupConnection trait handlers
// // Used by Mining Pools
// pub trait ConnectionServer {
//     fn on_setup_connection(&self, message: &common::SetupConnection) -> Result<()>;
// }

// // Used by downstream devices
// pub trait MiningConnectionClient {
//     fn on_mining_connection_success(&self, message: &mining::SetupConnectionSuccess) -> Result<()>;
//     fn on_mining_setup_connection_error(
//         &self,
//         message: &mining::SetupConnectionError,
//     ) -> Result<()>;
// }

// // NOTE: Mining Protocol
// pub trait ExtendedMiningServer {
//     fn on_open_extended_mining_channel(
//         &self,
//         message: &mining::OpenExtendedMiningChannel,
//     ) -> Result<()>;
// }

// pub trait ExtendedMiningClient {
//     fn on_open_extended_mining_channel_success(
//         &self,
//         message: &mining::OpenExtendedMiningChannelSuccess,
//     ) -> Result<()>;

//     fn on_open_extended_mining_channel_error(
//         &self,
//         message: &mining::OpenExtendedMiningChannelError,
//     ) -> Result<()>;
// }

// pub trait StandardMiningClient {}
// pub trait StandardMiningServer {}

// pub trait MiningServer {}
// pub trait MiningClient {}

// // NOTE: Network Message Handler
// // It should do something like:
// //   - match message.message_type {
// //   MessageType::OpenExtendedMiningChannel => self.on_open_extended_message(message)?;
// //   ... => Err("Don't handle this message")
// //   }
// pub trait NetworkMessageHandler {
//     fn receive_message(&self, message: &Message) -> Result<()>;
// }

// pub trait NetworkSender {
//     fn send_message(&self, message: &Message, some_networked_buffer?) -> Result<()>;
// }

// NOTE: Brainstorm draft
// pub struct PoolServer { ... }
//
// impl PoolServer {
//    MiningPoolHandler
// }
//
// receive_message() {
//   match MessageType::x | MessageType::y | ... | => MiningPoolHandler(message)
// }
//
// impl NetworkMessageHandler for PoolServer {
//    fn receive_message(&self, message: &Message) -> Result<()> {
//        match message.message_type() {
//            MessageType::OpenExtendedMiningChannel =>
//            self.on_open_extended_mining_channel(&unframe(&message))?,
//            _ => Err(Unimplemented)
//        }
//    }
// }
//
// impl NetworkSender for PoolServer {
//   fn send_message(&self, message: &Message, some_networked_buffer?) -> Result<()> {
//       some_networked_buffer.write(message)?
//   }
// }
//
// impl ConnectionServer for PoolServer {
//    fn on_setup_connection(&self, mesage: &common:SetupConnection) -> Result<()> {
//      // do stuff...
//      // Add a message on to a wire, maybe another trait?
//      self.send_message(some_new_message_response)?
//    }
// }
//
// impl ExtendedMiningServer for PoolServer {
//    fn on_open_extended_mining_channel(&self, mesage: &mining:OpenExtendedMiningChannel) -> Result<()> {
//      // do stuff...
//      // Add a message on to a wire, maybe another trait?
//      self.send_message(some_new_message_response)?
//    }
// }
