#[derive(Debug, Copy, Clone)]
enum Gender {
  #[allow(dead_code)]
  Unspecified = 0,
  Female = 1,
  Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug, Clone)]
struct User {
  id: UserId,
  name: String,
  gender: Gender,
}

#[derive(Debug, Clone)]
struct Topic {
  id: TopicId,
  name: String,
  owner: UserId,
}
 
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Event {
  Join((UserId, TopicId)),
  Leave((UserId, TopicId)),
  Message((UserId, TopicId, String)),
}

fn main() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };
    
    let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));
    
    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
}