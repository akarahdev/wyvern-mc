use crate::ConnectionHandle;
use std::fmt::Debug;
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
use voxidian_protocol::packet::c2s::status::C2SStatusPackets;
use voxidian_protocol::packet::s2c::status::{
    StatusResponse, StatusResponsePlayers, StatusResponseVersion,
};
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
                        let C2SHandshakePackets::Intention(packet) = packet;
                        let stage = packet.intended_stage.into_stage();
                        println!("new stage: {:?}", stage);
                        match stage {
                            Stage::Status => {
                                connection_handle.set_stage(Stage::Status);
                                println!("Connection is now status phase");
                            }
                            stage => connection_handle.set_stage(stage),
                        };
                    },
                );
            }
            Stage::Status => {
                self.parse_packets(
                    |packet: C2SStatusPackets, connection_handle: ConnectionHandle| match packet {
                        C2SStatusPackets::PingRequest(packet) => {}
                        C2SStatusPackets::StatusRequest(packet) => {
                            connection_handle
                                .send_packet(
                                    StatusResponse {
                                        version: StatusResponseVersion {
                                            name: "1.21.1".to_string(),
                                            protocol: 767,
                                        },
                                        players: StatusResponsePlayers {
                                            online: 0,
                                            max: 0,
                                            sample: vec![],
                                        },
                                        desc: Default::default(),
                                        favicon_png_b64: "".to_string(),
                                        enforce_chat_reports: false,
                                        prevent_chat_reports: false,
                                    }
                                    .to_packet(),
                                )
                                .unwrap();
                        }
                    },
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
