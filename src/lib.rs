use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use zenoh_interface::CmdVel;
use zenoh_manage_utils::{logger, param};
use async_std::{self, io::ReadExt};

use serde_json;

pub async fn wasd_controller(node_name:&str, yaml_path:&str)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let pub_topic = param::get_str_param(yaml_path, node_name, "pub_topic", "cmd_vel".to_string());
    let rate = param::get_f64_param(yaml_path, node_name, "power_rate", 1.0) as f32;

    let publisher = session.declare_publisher(pub_topic).res().await.unwrap();

    let mut stdin = async_std::io::stdin();
    let mut input = [0_u8];

    let msg = format!("Start pub:{}", publisher.key_expr().to_string());
    logger::log_info(node_name, msg);

    loop {
        let _ = stdin.read_exact(&mut input).await.unwrap();

        match input[0] {
            b'w'=>{
                let cmd = CmdVel{
                    x:0.0,
                    y:1.0*rate,
                    rotation_power:0.0
                };

                let serialized = serde_json::to_string(&cmd).unwrap();

                let msg = format!("send :{}", serialized);
                logger::log_info(node_name, msg);

                publisher.put(serialized).res().await.unwrap();
            },
            b'a'=>{
                let cmd = CmdVel{
                    x:-1.0*rate,
                    y:0.0,
                    rotation_power:0.0
                };

                let serialized = serde_json::to_string(&cmd).unwrap();

                let msg = format!("send :{}", serialized);
                logger::log_info(node_name, msg);

                publisher.put(serialized).res().await.unwrap();
            },
            b's'=>{
                let cmd = CmdVel{
                    x:0.0,
                    y:-1.0*rate,
                    rotation_power:0.0
                };

                let serialized = serde_json::to_string(&cmd).unwrap();

                let msg = format!("send :{}", serialized);
                logger::log_info(node_name, msg);

                publisher.put(serialized).res().await.unwrap();
            }
            b'd'=>{
                let cmd = CmdVel{
                    x:1.0*rate,
                    y:0.0,
                    rotation_power:0.0
                };

                let serialized = serde_json::to_string(&cmd).unwrap();

                let msg = format!("send :{}", serialized);
                logger::log_info(node_name, msg);

                publisher.put(serialized).res().await.unwrap();
            }
            0 => (),
            _ => (),
        }
    }
}