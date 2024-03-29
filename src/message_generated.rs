// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Event {
  NONE = 0,
  Login = 1,

}

const ENUM_MIN_EVENT: u8 = 0;
const ENUM_MAX_EVENT: u8 = 1;

impl<'a> flatbuffers::Follow<'a> for Event {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Event {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const Event;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const Event;
    unsafe { *p }
  }
}

impl flatbuffers::Push for Event {
    type Output = Event;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<Event>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_EVENT:[Event; 2] = [
  Event::NONE,
  Event::Login
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_EVENT:[&'static str; 2] = [
    "NONE",
    "Login"
];

pub fn enum_name_event(e: Event) -> &'static str {
  let index = e as u8;
  ENUM_NAMES_EVENT[index as usize]
}

pub struct EventUnionTableOffset {}
pub enum MessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
    type Inner = Message<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Message<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Message {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessageArgs) -> flatbuffers::WIPOffset<Message<'bldr>> {
      let mut builder = MessageBuilder::new(_fbb);
      if let Some(x) = args.event { builder.add_event(x); }
      builder.add_event_type(args.event_type);
      builder.finish()
    }

    pub const VT_EVENT_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_EVENT: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn event_type(&self) -> Event {
    self._tab.get::<Event>(Message::VT_EVENT_TYPE, Some(Event::NONE)).unwrap()
  }
  #[inline]
  pub fn event(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Message::VT_EVENT, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn event_as_login(&self) -> Option<Login<'a>> {
    if self.event_type() == Event::Login {
      self.event().map(|u| Login::init_from_table(u))
    } else {
      None
    }
  }

}

pub struct MessageArgs {
    pub event_type: Event,
    pub event: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for MessageArgs {
    #[inline]
    fn default() -> Self {
        MessageArgs {
            event_type: Event::NONE,
            event: None,
        }
    }
}
pub struct MessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_event_type(&mut self, event_type: Event) {
    self.fbb_.push_slot::<Event>(Message::VT_EVENT_TYPE, event_type, Event::NONE);
  }
  #[inline]
  pub fn add_event(&mut self, event: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_EVENT, event);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum LoginOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Login<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Login<'a> {
    type Inner = Login<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Login<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Login {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args LoginArgs<'args>) -> flatbuffers::WIPOffset<Login<'bldr>> {
      let mut builder = LoginBuilder::new(_fbb);
      if let Some(x) = args.pw { builder.add_pw(x); }
      if let Some(x) = args.id { builder.add_id(x); }
      builder.finish()
    }

    pub const VT_ID: flatbuffers::VOffsetT = 4;
    pub const VT_PW: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn id(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Login::VT_ID, None)
  }
  #[inline]
  pub fn pw(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Login::VT_PW, None)
  }
}

pub struct LoginArgs<'a> {
    pub id: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub pw: Option<flatbuffers::WIPOffset<&'a  str>>,
}
impl<'a> Default for LoginArgs<'a> {
    #[inline]
    fn default() -> Self {
        LoginArgs {
            id: None,
            pw: None,
        }
    }
}
pub struct LoginBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> LoginBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Login::VT_ID, id);
  }
  #[inline]
  pub fn add_pw(&mut self, pw: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Login::VT_PW, pw);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> LoginBuilder<'a, 'b> {
    let start = _fbb.start_table();
    LoginBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Login<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  flatbuffers::get_root::<Message<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  flatbuffers::get_size_prefixed_root::<Message<'a>>(buf)
}

#[inline]
pub fn finish_message_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_message_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
