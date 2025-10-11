use actix::prelude::*;
use std::sync::Arc;
use std::time::Duration;

/// ConsensusActor 负责管理共识算法的状态
pub struct ConsensusActor;

/// 定义共识算法的消息类型
pub enum ConsensusMessage {
    /// 节点加入网络
    JoinNetwork(String),
    /// 节点发送数据给其他节点
    SendData(Vec<u8>),
    /// 共识算法达成一致
    ConsensusReached(Vec<u8>),
}

impl Actor for ConsensusActor {
    type Context = Context<Self>;
}

/// 实现 ConsensusActor 的消息处理
impl StreamHandler<ConsensusMessage, actix::prelude::StreamHandshakeError<ConsensusMessage>> for ConsensusActor {
    fn handle(&mut self, msg: ConsensusMessage, ctx: &mut Self::Context) {
        match msg {
            ConsensusMessage::JoinNetwork(node_id) => {
                // 处理节点加入网络的逻辑
                println!("Node {} joined the network", node_id);
            },
            ConsensusMessage::SendData(data) => {
                // 处理节点发送数据的逻辑
                println!("Data received: {:?}