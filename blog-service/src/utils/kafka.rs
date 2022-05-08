use lazy_static::lazy_static;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
    util::Timeout, 
    ClientConfig
};
use std::{env::var, sync::Mutex, time::Duration};

lazy_static! { 
    static ref KAFKA_BROKER: String = 
        var("KAFKA_BROKER").expect("Cannot read Kafka Broker Address");
    
    static ref KAFKA_TOPIC: String = 
        var("KAFKA_TOPIC").expect("Cannot read Kafka Topic name");
}

pub(crate) fn create_producer() -> FutureProducer { 
    //   If there is an existing value for key in the configuration, it is overriden with the new value
    ClientConfig::new()
        .set("bootstrap.servers", KAFKA_BROKER.as_str())
        .set("message.timeout.ms", "5000")
        //  Create a Producer 
        .create()
        .expect("Producer Creation Failed")
}
pub(crate) fn create_consumer(group_id: String) -> StreamConsumer { 
    let consumer: StreamConsumer = ClientConfig::new() 
        .set("group.id", &group_id)
        .set("bootstrap.servers", KAFKA_BROKER.as_str())
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Could not create Consumer");
    consumer.subscribe(&[&KAFKA_TOPIC]).expect("Cannot Subscribe to specific topics");
    consumer

}
pub(crate) fn get_kafka_consumer_id(consumer_count: &Mutex<i32>) -> String { 
    let mut counter = consumer_count
        .lock()
        .expect("Cannot lock counter");
    *counter +=1;
    format!("Kafka Consumers: {}", *counter)
}


pub(crate) async fn send_message(producer: &FutureProducer, msg: String) { 
    let send_to_kafka = producer    
        .send(
            FutureRecord::to(&KAFKA_TOPIC)
                .payload(&msg)
                .key("New User Post"),
                Timeout::After(Duration::from_secs(0))
        ).await;
    send_to_kafka
        .map_or_else( 
            |err|
            println!("Message was not send: {}", err.0 ), 
            |_| println!("Message Sent"))
}