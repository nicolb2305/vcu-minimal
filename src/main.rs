use std::str::FromStr;

use futures_util::StreamExt;
use tokio::sync::mpsc::{self, Sender};
use tokio_tungstenite::tungstenite::Message;
use vcu_minimal::{
    api::{data::Champion, types::LolChampSelectChampSelectAction},
    client::Api,
    ws::types::{SubscriptionType, WebSocketEvent, WebSocketResponse},
    ws::WebSocketClient,
};

fn spawn_client_listener(mut client: WebSocketClient, tx: Sender<WebSocketResponse>) {
    let async_block = async move {
        while let Some(Ok(Message::Text(v))) = client.socket.next().await {
            if let Ok(val) = serde_json::from_str::<WebSocketEvent>(&v) {
                tx.send(val.data).await.unwrap();
            }
        }
    };
    tokio::spawn(async_block);
}

fn spawn_stdin_listener(tx: Sender<String>) {
    let async_block = async move {
        loop {
            let stdin = std::io::stdin();
            let mut line_buf = String::new();
            if stdin.read_line(&mut line_buf).is_ok() {
                let line = line_buf.trim_end().to_string();
                line_buf.clear();
                tx.send(line).await.unwrap();
            }
        }
    };
    tokio::spawn(async_block);
}

fn process_client_message(msg: WebSocketResponse) -> Vec<LolChampSelectChampSelectAction> {
    match msg {
        WebSocketResponse::ChampSelect(val) => {
            let picks = &val
                .data
                .champions()
                .iter()
                .map(|x| {
                    x.iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            dbg!(&val.data.bans);
            dbg!(picks);
            val.data.actions.into_iter().flatten().collect()
        }
        _ => unimplemented!(),
    }
}

async fn process_stdin_message(
    client: &Api,
    msg: &str,
    actions: &[LolChampSelectChampSelectAction],
) {
    let msg: String = msg
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let my_selection = client
        .get_lol_champ_select_v1_session_my_selection()
        .await
        .unwrap();

    if let Some(player) = actions
        .iter()
        .find(|x| x.actor_cell_id.unwrap() == my_selection.cell_id)
    {
        let champion = Champion::from_str(&msg);
        let action = match champion {
            Ok(Champion::None) => LolChampSelectChampSelectAction {
                completed: Some(true),
                ..Default::default()
            },
            Ok(champ) => LolChampSelectChampSelectAction {
                champion: Some(champ),
                ..Default::default()
            },
            Err(_) => return,
        };

        client
            .patch_lol_champ_select_v1_session_actions_by_id(player.id.unwrap(), action)
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let client = Api::new().unwrap();
    dbg!(client
        .get_lol_champ_select_v1_all_grid_champions()
        .await
        .unwrap());

    let (tx_ws, mut rx_ws) = mpsc::channel(5);
    let (tx_stdin, mut rx_stdin) = mpsc::channel(5);

    let mut ws_client = WebSocketClient::new().await.unwrap();
    ws_client
        .subscribe(SubscriptionType::LolChampSelectV1Session)
        .await
        .unwrap();

    spawn_client_listener(ws_client, tx_ws);
    spawn_stdin_listener(tx_stdin);

    let mut actions: Vec<LolChampSelectChampSelectAction> = vec![];
    loop {
        tokio::select! {
            Some(msg) = rx_ws.recv() => {
                dbg!(&msg);
                actions = process_client_message(msg);
                // dbg!(&actions);
            },
            Some(champion_name) = rx_stdin.recv() => {
                process_stdin_message(&client, &champion_name, &actions).await;
            }
        }
    }
}
