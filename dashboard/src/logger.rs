use core::slog::{self, Drain, Level, OwnedKVList, KV};
use std::fmt::{self, Display, Formatter};
use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct InMemoryDrain {
    messages: Mutex<Vec<Record>>,
}

impl Drain for InMemoryDrain {
    type Ok = ();
    type Err = slog::Error;

    fn log(
        &self,
        record: &slog::Record<'_>,
        kv: &OwnedKVList,
    ) -> Result<Self::Ok, Self::Err> {
        let mut messages = self.messages.lock().unwrap();

        let mut ser = Serializer::default();
        kv.serialize(record, &mut ser)?;

        let mut record = Record::from(record);
        record.kv.extend(ser.0);

        messages.push(record);
        Ok(())
    }
}

impl Display for InMemoryDrain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let messages = self.messages.lock().unwrap();

        for record in messages.iter() {
            write!(f, "[{}]", record.level.to_string().to_uppercase())?;

            if record.message.len() > 0 {
                write!(f, " {}", record.message)?;
            }

            for (i, (key, value)) in record.kv.iter().enumerate() {
                if i > 0 || !record.message.is_empty() {
                    write!(f, ", ")?;
                } else {
                    // first key-value pair and the message was empty. Add
                    // some space between the log level and key-value pairs.
                    write!(f, " ")?;
                }

                write!(f, "{} = {}", key, value)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Record {
    level: Level,
    message: String,
    tag: String,
    kv: Vec<(&'static str, String)>,
}

impl<'a> From<&'a slog::Record<'_>> for Record {
    fn from(other: &'a slog::Record<'_>) -> Record {
        let mut pairs = Serializer::default();
        other.kv().serialize(other, &mut pairs).unwrap();

        Record {
            level: other.level(),
            message: other.msg().to_string(),
            tag: other.tag().to_string(),
            kv: pairs.0,
        }
    }
}

#[derive(Default)]
struct Serializer(Vec<(&'static str, String)>);

impl slog::Serializer for Serializer {
    fn emit_arguments(
        &mut self,
        key: &'static str,
        value: &std::fmt::Arguments<'_>,
    ) -> slog::Result {
        self.0.push((key, value.to_string()));
        Ok(())
    }
}
