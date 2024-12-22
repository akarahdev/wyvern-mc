use crate::ConnectionHandle;
use std::fmt::Debug;
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
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
        let stage = self.get_stage().clone();

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
            _ => {}
        }
    }

    pub(crate) fn parse_packets<T: PrefixedPacketDecode + Debug, F: Fn(T, ConnectionHandle)>(
        &self,
        f: F,
    ) {
        let mut inner = self.inner.lock().unwrap();
        let handle = self.clone();
        // TODO: remove this clone PLEASE this will kill performance if you don't oh my god
        let b = inner.incoming_bytes.clone();
        let a = b.iter().map(|x| *x).clone();
        match inner.packet_processing.decode_from_raw_queue(a) {
            Ok((mut buf, consumed)) => {
                for _ in 0..consumed {
                    inner.incoming_bytes.pop_front();
                }
                match T::decode_prefixed(&mut buf) {
                    Ok(packet) => {
                        drop(inner);
                        f(packet, handle)
                    }
                    Err(DecodeError::EndOfBuffer) => {
                        drop(inner);
                        self.mark_for_removal();
                        return;
                    }
                    Err(e) => panic!("{:?}", e),
                }
            }
            Err(DecodeError::EndOfBuffer) => {
                drop(inner);
                self.mark_for_removal();
                return;
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
