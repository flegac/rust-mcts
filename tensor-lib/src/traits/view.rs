use crate::structs::offset4::Offset4;
use crate::structs::shape4::Shape4;
use crate::structs::view4::View4;

pub trait View {
    fn offset(&self) -> &Offset4;
    fn shape(&self) -> &Shape4;

    fn to_parent(&self, parent: &Shape4, offset: &Offset4) -> Offset4 {
        let mut res = offset + self.offset();
        parent.check(&res);
        res
    }

    fn clone_view(&self) -> View4 {
        View4 {
            offset: self.offset().clone(),
            shape: self.shape().clone(),
        }
    }
}
