extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate byteorder;

#[cfg(test)]
extern crate tempdir;

pub mod log;

#[cfg(test)]
mod test {
    use tempdir::TempDir;
    use super::log::{Log, Consumer, Record};

    #[test]
    fn basic_write_then_read() {
        let dir = TempDir::new_in(".", "logs").expect("creating tempdir");

        let mut log = Log::new(&dir).expect("failed to build log");
        let mut consumer = Consumer::new(&dir).expect("failed to build consumer");

        let batch_in = vec![
            Record::new("i am the first message"),
            Record::new("i am the second message"),
        ];

        log.append(&batch_in).expect("failed to append batch");

        let batch_out = consumer.poll().expect("failed to poll for batch");
        assert_eq!(batch_in, batch_out);
    }

    #[test]
    fn consumer_starts_from_the_end() {
        let dir = TempDir::new_in(".", "logs").expect("creating tempdir");

        let mut log = Log::new(&dir).expect("failed to build log");

        let first_batch = vec![
            Record::new("i am the first message"),
            Record::new("i am the second message"),
        ];
        log.append(&first_batch).expect("failed to append batch");

        let mut consumer = Consumer::new(&dir).expect("failed to build consumer");

        let second_batch = vec![
            Record::new("i am the third message"),
            Record::new("i am the fourth message"),
        ];
        log.append(&second_batch).expect("failed to append batch");

        let batch_out = consumer.poll().expect("failed to poll for batch");
        assert_eq!(second_batch, batch_out);
    }

    #[test]
    fn logs_split_into_segments() {
        let dir = TempDir::new_in(".", "logs").expect("creating tempdir");

        let mut log = Log::new(&dir).expect("failed to build log");

        let batch = vec![Record::new("i am the first message")];
        log.append(&batch).expect("failed to append batch");

        // make this auto with config?
        log.roll_segment().expect("failed to roll new segment");

        let batch = vec![Record::new("i am the second message")];
        log.append(&batch).expect("failed to append batch");

        assert_eq!(2, ::std::fs::read_dir(&dir).unwrap().count());
    }
}