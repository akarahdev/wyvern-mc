use voxidian_protocol::{
    packet::{
        Stage,
        c2s::login::C2SLoginPackets,
        s2c::{
            config::{CustomPayloadS2CConfigPacket, KnownPack, SelectKnownPacksS2CConfigPacket},
            login::{LoginFinishedS2CLoginPacket, LoginSuccessProperty},
        },
    },
    value::{ConsumeAllVec, Identifier, LengthPrefixHashMap, LengthPrefixVec, VarInt},
};

use crate::{plugin::Plugin, ServerBuilder};

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn load(&self, server: &ServerBuilder) {
        server.low_level(|server| {
            server.login_event(|packet, connection| {
                let C2SLoginPackets::Hello(packet) = packet else {
                    return;
                };
                let connection = connection.raw_handle();
                let mut props = LengthPrefixHashMap::<VarInt, String, LoginSuccessProperty>::new();
                props.insert(
                    "textures".into(),
                    LoginSuccessProperty {
                        value: "ewogICJ0aW1lc3RhbXAiIDogMTYxMjIxMTAxNDg1MywKICAicHJvZmlsZUlkIiA6ICI1ZWE0ODg2NTg2OWI0Y2ZhOWRjNTg5YmFlZWQwNzM5MCIsCiAgInByb2ZpbGVOYW1lIiA6ICJfUllOMF8iLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvN2NmNDU1YmI4NjcyN2M1NjFlNjI2ZDIxZjA3MGE1OTdmMDlhOTZkOGFhNmMwZmRjM2JjYjZkMDE2NTZjMDk3OCIKICAgIH0KICB9Cn0=".to_string(),
                        sig: Some("SA3W+MXMEWPOwmktk2K8G9kYSb07loa/UOCqBF7PBlvMzGrPb7clNQS/JP2uXU3BxlunguuLPK2bR+Q86neBBSzndSErB8oyJorogi/1y0LOEFVF98Iy0hGrDDCuuT+236SY2L+u05Y/cpN7M/lE4J2YLitx7RzWfqcdxIJE8nCcJcfso1YKEMHzKlkQkxtZOd5+HDfmAlI9qSaK0LpgEFF5DieYMhRvbC6Vl54AXTfTYMZ1QmixmxdBXMSF1sDWzl57Jx79Q6djB/BahMC9aj83rTcyZJaXJS6PqVOULx7YZFs89abVtzrj+pvt3b2SMZoEbjOMsGulXy336NJBuf7mPN+MXz2bnwGbhxYwDrMdSwUjgm+iH9XWwN3piAovenhRyW4vdpXVYf4993gnQBbOVyDFmf/COLt5mezsSNTmCMkoEXrdvz02JjzxmzXasv25rglPSlZFWmStrEMGTHARLtNvKF+SL5LYiHl8rBJrvQDEOSj0fR3eH9o+MSlT5veNjdtDFt2Llc+0tiSqvuM1e3PnE72ALC6cPDludDQI9+YFbX5uV1miB0C0Fe/+DEGe3oVtufP122yobEB1fegWf02BZtCp4Ss8Zm8JOQepXhOvw7QjJFyRckZRHa0GlkBdMYr5GHNe9cTtPEUEAOwrQ86eqo/jk/IFMChiNvY=".to_string()),
                    },
                );
                connection.send_packet(LoginFinishedS2CLoginPacket {
                    uuid: packet.uuid,
                    username: packet.username.clone(),
                    props,
                }).unwrap();
            });
            server.login_event(|packet, connection| {
                let C2SLoginPackets::LoginAcknowledged(_packet) = packet else {
                    return;
                };
                let connection = connection.raw_handle();
                connection.set_stage(Stage::Config);
                let mut data = ConsumeAllVec::new();
                data.extend("Wyvern-MC".bytes());
                connection.send_packet(CustomPayloadS2CConfigPacket {
                    channel: Identifier::new("minecraft", "branc"),
                    data,
                }).unwrap();
                let mut known_packs = LengthPrefixVec::new();
                known_packs.push(KnownPack {
                    namespace: "minecraft".to_string(),
                    id: "core".to_string(),
                    version: "1.21.4".to_string(),
                });
                known_packs.push(KnownPack {
                    namespace: "minecraft".to_string(),
                    id: "vanilla".to_string(),
                    version: "1.21.4".to_string(),
                });
                connection.send_packet(SelectKnownPacksS2CConfigPacket {
                    known_packs,
                }).unwrap();
            });
        });
    }
}
