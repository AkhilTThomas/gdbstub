use managed::ManagedSlice;

use crate::util::managed_vec::{CapacityError, ManagedVec};

enum State {
    Ready,
    Body,
    Checksum1,
    Checksum2,
}

/// Receives a packet incrementally using a asynchronous state machine.
pub struct RecvPacketStateMachine {
    state: State,
    idx: usize,
}

impl RecvPacketStateMachine {
    pub fn new() -> Self {
        RecvPacketStateMachine {
            state: State::Ready,
            idx: 0,
        }
    }

    pub fn pump<'b>(
        &mut self,
        packet_buffer: &'b mut ManagedSlice<u8>,
        byte: u8,
    ) -> Result<Option<&'b mut [u8]>, CapacityError<u8>> {
        let mut buf = ManagedVec::new_with_idx(packet_buffer, self.idx);
        buf.push(byte)?;
        self.idx += 1;

        match self.state {
            State::Ready => {
                if byte == b'$' {
                    self.state = State::Body;
                } else {
                    self.idx = 0;
                }
            }
            State::Body => {
                if byte == b'#' {
                    self.state = State::Checksum1;
                }
            }
            State::Checksum1 => self.state = State::Checksum2,
            State::Checksum2 => {
                self.state = State::Ready;
                self.idx = 0;
            }
        }

        if matches!(self.state, State::Ready) {
            #[cfg(feature = "trace-pkt")]
            trace!(
                "<-- {}",
                core::str::from_utf8(buf.as_slice()).unwrap_or("<invalid packet>")
            );

            Ok(Some(packet_buffer))
        } else {
            Ok(None)
        }
    }
}
