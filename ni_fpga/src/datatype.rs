use crate::ffi;
use crate::Offset;
use crate::Session;
use crate::Status;

pub trait Datatype: Sized {
    fn read(
        session: Session,
        offset: Offset,
    ) -> Result<Self, Status>;
    fn write(
        session: Session,
        offset: Offset,
        value: Self,
    ) -> Result<(), Status>;
}

impl Datatype for bool {
    fn read(
        session: Session,
        offset: Offset,
    ) -> Result<Self, Status> {
        let mut value: bool = Default::default();
        let status = Status::from(unsafe {
            ffi::ReadBool(
                session.handle,
                offset,
                &mut value as *mut bool,
            )
        });
        match status {
            Status::Success => Ok(value),
            _ => Err(status),
        }
    }
    fn write(
        session: Session,
        offset: Offset,
        value: Self,
    ) -> Result<(), Status> {
        let status = Status::from(unsafe {
            ffi::WriteBool(
                session.handle,
                offset,
                value,
            )
        });
        match status {
            Status::Success => Ok(()),
            _ => Err(status),
        }
    }
}
