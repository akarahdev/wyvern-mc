use crate::Player;
use std::fmt::Debug;
use voxidian_protocol::packet::c2s::config::C2SConfigPackets;
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
use voxidian_protocol::packet::c2s::login::C2SLoginPackets;
use voxidian_protocol::packet::c2s::play::C2SPlayPackets;
use voxidian_protocol::packet::c2s::status::C2SStatusPackets;
use voxidian_protocol::packet::{DecodeError, PrefixedPacketDecode, Stage};

use super::protocol::RawConnection;

impl RawConnection {
    pub fn mark_for_removal(&self) {
        self.inner.lock().unwrap().mark_for_removal = true;
    }

    pub fn marked_for_removal(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.mark_for_removal
    }

    pub fn handle_incoming_data(&self) {
        self.inner.lock().unwrap().handle_incoming_data();
        let stage = self.get_stage();

        match stage {
            Stage::Handshake => {
                self.parse_packets(
                    |packet: C2SHandshakePackets, connection_handle: Player| {
                        for event in self.server.get_low_level().handshake_events() {
                            event(&packet, connection_handle.clone())
                        }
                    },
                );
            }
            Stage::Status => {
                self.parse_packets(
                    |packet: C2SStatusPackets, connection_handle: Player| {
                        for event in self.server.get_low_level().status_events() {
                            event(&packet, connection_handle.clone());
                        }
                    },
                );
            }
            Stage::Login => {
                self.parse_packets(
                    |packet: C2SLoginPackets, connection_handle: Player| {
                        for event in self.server.get_low_level().login_events() {
                            event(&packet, connection_handle.clone());
                        }
                    },
                );
            }
            Stage::Config => {
                self.parse_packets(
                    |packet: C2SConfigPackets, connection_handle: Player| {
                        for event in self.server.get_low_level().configuration_events() {
                            event(&packet, connection_handle.clone());
                        }
                    },
                );
            }
            Stage::Play => {
                self.parse_packets(
                    |packet: C2SPlayPackets, connection_handle: Player| {
                        for event in self.server.get_low_level().play_events() {
                            event(&packet, connection_handle.clone());
                        }
                    },
                );
            }
            _ => {}
        }

        self.inner.lock().unwrap().handle_outgoing_data();
    }

    pub fn parse_packets<T: PrefixedPacketDecode + Debug, F: Fn(T, Player)>(&self, f: F) {
        let mut inner = self.inner.lock().unwrap();
        let handle = self.clone();

        let bytes = inner.incoming_bytes.iter().copied().collect::<Vec<_>>();

        match inner
            .packet_processing
            .decode_from_raw_queue(bytes.into_iter())
        {
            Ok((mut buf, consumed)) => {
                if consumed == 0 {
                    return;
                }
                for _ in 0..consumed {
                    inner.incoming_bytes.pop_front();
                }
                match T::decode_prefixed(&mut buf) {
                    Ok(packet) => {
                        drop(inner);
                        f(packet, handle.to_safe())
                    }
                    Err(DecodeError::EndOfBuffer) => {}
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
            Err(DecodeError::EndOfBuffer) => {}
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
