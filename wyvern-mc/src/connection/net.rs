use crate::ConnectionHandle;
use std::fmt::Debug;
use voxidian_protocol::packet::c2s::config::C2SConfigPackets;
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
use voxidian_protocol::packet::c2s::login::C2SLoginPackets;
use voxidian_protocol::packet::c2s::status::C2SStatusPackets;
use voxidian_protocol::packet::{DecodeError, PrefixedPacketDecode, Stage};

impl ConnectionHandle {
    pub(crate) fn mark_for_removal(&self) {
        self.inner.lock().unwrap().mark_for_removal = true;
    }

    pub(crate) fn marked_for_removal(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        *&inner.mark_for_removal
    }

    pub(crate) fn handle_incoming_data(&self) {
        self.inner.lock().unwrap().handle_incoming_data();
        let stage = self.get_stage();

        match stage {
            Stage::Handshake => {
                self.parse_packets(
                    |packet: C2SHandshakePackets, connection_handle: ConnectionHandle| {
                        for event in self.server.handshake_events() {
                            event(&packet, connection_handle.clone())
                        }
                    },
                );
            }
            Stage::Status => {
                self.parse_packets(
                    |packet: C2SStatusPackets, connection_handle: ConnectionHandle| {
                        for event in self.server.status_events() {
                            event(&packet, connection_handle.clone());
                        }
                    }
                );
            }
            Stage::Login => {
                self.parse_packets(
                    |packet: C2SLoginPackets, connection_handle: ConnectionHandle| {
                        for event in self.server.login_events() {
                            event(&packet, connection_handle.clone());
                        }
                    }
                );
            }
            Stage::Config => {
                self.parse_packets(
                    |packet: C2SConfigPackets, connection_handle: ConnectionHandle| {
                        for event in self.server.configuration_events() {
                            event(&packet, connection_handle.clone());
                        }
                    }
                );
            }
            _ => {}
        }

        self.inner.lock().unwrap().handle_outgoing_data();
    }

    pub(crate) fn parse_packets<T: PrefixedPacketDecode + Debug, F: Fn(T, ConnectionHandle)>(
        &self,
        f: F,
    ) {
        let mut inner = self.inner.lock().unwrap();
        let handle = self.clone();

        if inner.incoming_bytes.len() > 0 {
            println!("len: {:?}", inner.incoming_bytes.len());
        }
        let bytes = inner
            .incoming_bytes
            .iter()
            .map(|x| *x)
            .collect::<Vec<_>>();

        match inner.packet_processing.decode_from_raw_queue(bytes.into_iter()) {
            Ok((mut buf, consumed)) => {
                if consumed == 0 {
                    return;
                }
                println!("slen: {:?}", inner.incoming_bytes.len());
                println!("consumed: {:?}", consumed);
                for _ in 0..consumed {
                    inner.incoming_bytes.pop_front();
                }
                println!("olen: {:?}", inner.incoming_bytes.len());
                println!("IN: {:?}", buf);
                match T::decode_prefixed(&mut buf) {
                    Ok(packet) => {
                        drop(inner);
                        f(packet, handle)
                    }
                    Err(DecodeError::EndOfBuffer) => {}
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
            Err(DecodeError::EndOfBuffer) => {
                drop(inner);
                return;
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
