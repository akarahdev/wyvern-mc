use core::panic;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::{
    net::TcpListener,
    sync::mpsc::{Receiver, Sender},
};

use crate::{
    dimension::{Dimension, DimensionData},
    player::{net::ConnectionData, player::ConnectionWithSignal},
    systems::{
        events::ServerTickEvent,
        parameters::{Event, Param},
        system::System,
        typemap::TypeMap,
    },
    values::Key,
};

use super::{
    Server, dimensions::DimensionContainer, message::ServerMessage, registries::RegistryContainer,
};

pub struct ServerData {
    pub(crate) connections: Vec<ConnectionWithSignal>,
    pub(crate) systems: Vec<Box<dyn System + Send + Sync + 'static>>,
    pub(crate) registries: RegistryContainer,
    pub(crate) dimensions: DimensionContainer,
    pub(crate) last_tick: Instant,
}

impl ServerData {
    pub async fn start(mut self) {
        let (tx, rx) = tokio::sync::mpsc::channel::<ServerMessage>(16);
        let root_dim = DimensionData::new(
            Key::new("wyvern", "root"),
            Server { sender: tx.clone() },
            Key::new("minecraft", "overworld"),
        );

        self.dimensions
            .insert(Key::new("wyvern", "root"), Dimension {
                tx: root_dim.tx.clone(),
                server: Server { sender: tx.clone() },
            });

        tokio::spawn(root_dim.handle_messages());
        tokio::spawn(self.handle_loops(tx.clone(), rx));
        tokio::spawn(Self::networking_loop(tx));
    }

    pub async fn handle_loops(
        mut self,
        tx: Sender<ServerMessage>,
        mut rx: Receiver<ServerMessage>,
    ) {
        loop {
            self.connections
                .retain_mut(|connection| connection._signal.try_recv().is_err());

            for system in &mut self.systems {
                let mut map = TypeMap::new();
                let server = Server { sender: tx.clone() };
                if let Some(fut) = system.run(&mut map, server) {
                    tokio::spawn(fut);
                }
            }

            self.handle_messages(&tx, &mut rx).await;

            let dur = Instant::now().duration_since(self.last_tick);
            if dur > Duration::from_millis(50) {
                self.last_tick = Instant::now();

                let server = Server { sender: tx.clone() };
                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    tx_clone
                        .send(ServerMessage::FireSystems({
                            let mut map = TypeMap::new();
                            map.insert(Event::<ServerTickEvent>::new());
                            map.insert(Param::new(server));
                            map
                        }))
                        .await
                });
            }
        }
    }

    pub async fn handle_messages(
        &mut self,
        tx: &Sender<ServerMessage>,
        rx: &mut Receiver<ServerMessage>,
    ) {
        if let Ok(msg) = rx.try_recv() {
            match msg {
                ServerMessage::SpawnConnection(connection_with_signal) => {
                    self.connections.push(connection_with_signal);
                }
                ServerMessage::FireSystems(mut parameters) => {
                    for system in &mut self.systems {
                        let server = Server { sender: tx.clone() };
                        if let Some(fut) = system.run(&mut parameters, server) {
                            tokio::spawn(fut);
                        }
                    }
                }
                ServerMessage::DamageTypeRegistry(sender) => {
                    match sender.send(self.registries.damage_types.clone()) {
                        Ok(()) => {}
                        Err(_e) => panic!("DID NOT SEND AAA"),
                    };
                }
                ServerMessage::BiomeRegistry(sender) => {
                    match sender.send(self.registries.biomes.clone()) {
                        Ok(()) => {}
                        Err(_e) => panic!("DID NOT SEND AAA"),
                    };
                }
                ServerMessage::WolfRegistry(sender) => {
                    match sender.send(self.registries.wolf_variants.clone()) {
                        Ok(()) => {}
                        Err(_e) => panic!("DID NOT SEND AAA"),
                    };
                }
                ServerMessage::PaintingRegistry(sender) => {
                    match sender.send(self.registries.painting_variants.clone()) {
                        Ok(()) => {}
                        Err(_e) => panic!("DID NOT SEND AAA"),
                    };
                }
                ServerMessage::DimTypeRegistry(sender) => {
                    match sender.send(self.registries.dimension_types.clone()) {
                        Ok(()) => {}
                        Err(_e) => panic!("DID NOT SEND AAA"),
                    };
                }
                ServerMessage::GetDimension(key, sender) => {
                    match sender.send(self.dimensions.get(&key).cloned()) {
                        Ok(()) => {}
                        Err(_e) => panic!("DID NOT SEND AAA"),
                    }
                }
            }
        };
    }

    pub async fn networking_loop(tx: Sender<ServerMessage>) {
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 25565))
            .await
            .unwrap();

        println!("Server now listening on 127.0.0.1:25565");
        loop {
            let new_client = listener.accept().await;
            match new_client {
                Ok((stream, addr)) => {
                    println!("Accepted new client: {:?}", addr);

                    let server = Server { sender: tx.clone() };
                    let (messenger, signal) =
                        ConnectionData::connection_channel(stream, addr.ip(), server);
                    let proxy = ConnectionWithSignal {
                        messenger: Arc::new(messenger),
                        _signal: signal,
                    };
                    let _lowered = proxy.lower();
                    tx.send(ServerMessage::SpawnConnection(proxy))
                        .await
                        .unwrap();
                }
                Err(_err) => {}
            }
        }
    }
}
