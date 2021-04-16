use {
  std::vec,
  std::iter,
};

#[derive(Debug, Copy, Clone, PartialEq)]
enum NoteKind12Edo {
  A,
  ASharp,
  B,
  C,
  CSharp,
  D,
  DSharp,
  E,
  F,
  FSharp,
  G,
  GSharp,
}

impl Default for NoteKind12Edo {
  fn default() -> Self { NoteKind12Edo::A }
}

fn toindex_12edo(value : NoteKind12Edo) -> i8 {
  match value {
    NoteKind12Edo::A => 1,
    NoteKind12Edo::ASharp => 2,
    NoteKind12Edo::B => 3,
    NoteKind12Edo::C => 4,
    NoteKind12Edo::CSharp => 5,
    NoteKind12Edo::D => 6,
    NoteKind12Edo::DSharp => 7,
    NoteKind12Edo::E => 8,
    NoteKind12Edo::F => 9,
    NoteKind12Edo::FSharp => 10,
    NoteKind12Edo::G => 11,
    NoteKind12Edo::GSharp => 12,
  }
}

fn fromindex_12edo(index : i8) -> Option<NoteKind12Edo> {
  match index {
    1 => Some(NoteKind12Edo::A),
    2 => Some(NoteKind12Edo::ASharp),
    3 => Some(NoteKind12Edo::B),
    4 => Some(NoteKind12Edo::C),
    5 => Some(NoteKind12Edo::CSharp),
    6 => Some(NoteKind12Edo::D),
    7 => Some(NoteKind12Edo::DSharp),
    8 => Some(NoteKind12Edo::E),
    9 => Some(NoteKind12Edo::F),
    10 => Some(NoteKind12Edo::FSharp),
    11 => Some(NoteKind12Edo::G),
    0 => Some(NoteKind12Edo::GSharp),
    _ => None,
  }      
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct NoteName12Edo {
  kind : NoteKind12Edo,
  octave : i8,
}

fn transpose(value : NoteName12Edo, interval : i8) -> NoteName12Edo {
  let mut ret : NoteName12Edo = NoteName12Edo::default();
  ret.kind = fromindex_12edo((toindex_12edo(value.kind) + interval ) % 12).unwrap();
  ret.octave = value.octave + ((toindex_12edo(value.kind) + interval ) / 12);
  ret
}

type Chord = Vec<NoteName12Edo>;

type Voicings = Vec<i8>;

fn voicings_to_chord(root : NoteName12Edo, voicings : Voicings) -> Chord {
  let mut ret : Chord = Vec::new();
  for index in 0..voicings.len() {
    ret.push(transpose(root, *voicings.get(index).unwrap()));
  }
  ret
}

fn main() {

}


#[test]
fn test() {
  assert_eq!(fromindex_12edo(1).unwrap(), NoteKind12Edo::A);
  assert_eq!(toindex_12edo(NoteKind12Edo::B), 3);
  assert_eq!(toindex_12edo(NoteKind12Edo::A), 1);
  assert_eq!(transpose(NoteName12Edo{kind : NoteKind12Edo::C, octave : 2}, 7), NoteName12Edo{kind : NoteKind12Edo::G, octave : 2});
  assert_eq!(transpose(NoteName12Edo{kind : NoteKind12Edo::A, octave : 2}, 22), NoteName12Edo{kind : NoteKind12Edo::G, octave : 3});
  assert_eq!([1, 4, 7].to_vec().len(), 3);
  assert_eq!(voicings_to_chord(NoteName12Edo{kind : NoteKind12Edo::C, octave : 2}, [0, 4, 7].to_vec()),
             [NoteName12Edo{kind : NoteKind12Edo::C, octave : 2},
              NoteName12Edo{kind : NoteKind12Edo::E, octave : 2},
              NoteName12Edo{kind : NoteKind12Edo::G, octave : 2}]);
}
