use primitiv_sys as _primitiv;
use std::ops;
use std::ptr;
use device::{AnyDevice, Device};
use Node;
use Parameter;
use Shape;
use Status;
use Wrap;

macro_rules! impl_node_scalar_op {
    ($scalar:ty,
     $name:ident,
     $op_fn:ident,
     $api_nc_fn:ident,
     $api_cn_fn:ident) => {
        impl ops::$name<$scalar> for Node {
            type Output = Node;

            fn $op_fn(self, rhs: $scalar) -> Node {
                let mut status = Status::new();
                unsafe {
                    let node = Node::from_inner_ptr(_primitiv::$api_nc_fn(
                        self.as_inner_ptr(),
                        rhs as f32,
                        status.as_inner_mut_ptr(),
                    ));
                    status.into_result().unwrap();
                    node
                }
            }
        }

        impl ops::$name<Node> for $scalar {
            type Output = Node;

            fn $op_fn(self, rhs: Node) -> Node {
                let mut status = Status::new();
                unsafe {
                    let node = Node::from_inner_ptr(_primitiv::$api_cn_fn(
                        self as f32,
                        rhs.as_inner_ptr(),
                        status.as_inner_mut_ptr(),
                    ));
                    status.into_result().unwrap();
                    node
                }
            }
        }
    }
}

macro_rules! impl_bin_node_op {
    ($name:ident,
     $op_fn:ident,
     $api_nc_fn:ident,
     $api_cn_fn:ident,
     $api_nn_fn:ident) => {
        impl_node_scalar_op!(i8, $name, $op_fn, $api_nc_fn, $api_cn_fn);
        impl_node_scalar_op!(u8, $name, $op_fn, $api_nc_fn, $api_cn_fn);
        impl_node_scalar_op!(i16, $name, $op_fn, $api_nc_fn, $api_cn_fn);
        impl_node_scalar_op!(u16, $name, $op_fn, $api_nc_fn, $api_cn_fn);
        impl_node_scalar_op!(i32, $name, $op_fn, $api_nc_fn, $api_cn_fn);
        impl_node_scalar_op!(u32, $name, $op_fn, $api_nc_fn, $api_cn_fn);
        impl_node_scalar_op!(i64, $name, $op_fn, $api_nc_fn, $api_cn_fn);
        impl_node_scalar_op!(u64, $name, $op_fn, $api_nc_fn, $api_cn_fn);
        impl_node_scalar_op!(f32, $name, $op_fn, $api_nc_fn, $api_cn_fn);
        impl_node_scalar_op!(f64, $name, $op_fn, $api_nc_fn, $api_cn_fn);

        impl ops::$name<Node> for Node {
            type Output = Node;

            fn $op_fn(self, rhs: Node) -> Node {
                let mut status = Status::new();
                unsafe {
                    let node = Node::from_inner_ptr(_primitiv::$api_nn_fn(
                        self.as_inner_ptr(),
                        rhs.as_inner_ptr(),
                        status.as_inner_mut_ptr(),
                    ));
                    status.into_result().unwrap();
                    node
                }
            }
        }

        impl<'a> ops::$name<Node> for &'a Node {
            type Output = Node;

            fn $op_fn(self, rhs: Node) -> Node {
                let mut status = Status::new();
                unsafe {
                    let node = Node::from_inner_ptr(_primitiv::$api_nn_fn(
                        self.as_inner_ptr(),
                        rhs.as_inner_ptr(),
                        status.as_inner_mut_ptr(),
                    ));
                    status.into_result().unwrap();
                    node
                }
            }
        }

        impl<'a> ops::$name<&'a Node> for Node {
            type Output = Node;

            fn $op_fn(self, rhs: &'a Node) -> Node {
                let mut status = Status::new();
                unsafe {
                    let node = Node::from_inner_ptr(_primitiv::$api_nn_fn(
                        self.as_inner_ptr(),
                        rhs.as_inner_ptr(),
                        status.as_inner_mut_ptr(),
                    ));
                    status.into_result().unwrap();
                    node
                }
            }
        }

        impl<'a, 'b> ops::$name<&'a Node> for &'b Node {
            type Output = Node;

            fn $op_fn(self, rhs: &'a Node) -> Node {
                let mut status = Status::new();
                unsafe {
                    let node = Node::from_inner_ptr(_primitiv::$api_nn_fn(
                        self.as_inner_ptr(),
                        rhs.as_inner_ptr(),
                        status.as_inner_mut_ptr(),
                    ));
                    status.into_result().unwrap();
                    node
                }
            }
        }

    }
}

impl_bin_node_op!(
    Add,
    add,
    safe_primitiv_node_func_add_node_const,
    safe_primitiv_node_func_add_const_node,
    safe_primitiv_node_func_add_node_node
);
impl_bin_node_op!(
    Sub,
    sub,
    safe_primitiv_node_func_subtract_node_const,
    safe_primitiv_node_func_subtract_const_node,
    safe_primitiv_node_func_subtract_node_node
);
impl_bin_node_op!(
    Mul,
    mul,
    safe_primitiv_node_func_multiply_node_const,
    safe_primitiv_node_func_multiply_const_node,
    safe_primitiv_node_func_multiply_node_node
);
impl_bin_node_op!(
    Div,
    div,
    safe_primitiv_node_func_divide_node_const,
    safe_primitiv_node_func_divide_const_node,
    safe_primitiv_node_func_divide_node_node
);

pub fn input<S: Into<Shape>>(shape: S, data: &[f32]) -> Node {
    input_with_device::<S, AnyDevice>(shape, data, None)
}

pub fn input_with_device<S: Into<Shape>, D: Device>(
    shape: S,
    data: &[f32],
    device: Option<&mut D>,
) -> Node {
    let mut status = Status::new();
    unsafe {
        let node = Node::from_inner_ptr(_primitiv::safe_primitiv_node_func_input(
            shape.into().as_inner_ptr(),
            data.as_ptr() as *const _,
            data.len(),
            device.map(|d| d.as_inner_mut_ptr()).unwrap_or(
                ptr::null_mut(),
            ),
            ptr::null_mut(),
            status.as_inner_mut_ptr(),
        ));
        status.into_result().unwrap();
        node
    }
}

pub fn parameter(param: &mut Parameter) -> Node {
    let mut status = Status::new();
    unsafe {
        let node = Node::from_inner_ptr(_primitiv::safe_primitiv_node_func_parameter(
            param.as_inner_mut_ptr(),
            ptr::null_mut(),
            status.as_inner_mut_ptr(),
        ));
        status.into_result().unwrap();
        node
    }
}

pub fn matmul<N: AsRef<Node>>(a: N, b: N) -> Node {
    let mut status = Status::new();
    unsafe {
        let node = Node::from_inner_ptr(_primitiv::safe_primitiv_node_func_matmul(
            a.as_ref().as_inner_ptr(),
            b.as_ref().as_inner_ptr(),
            status.as_inner_mut_ptr(),
        ));
        status.into_result().unwrap();
        node
    }
}

pub fn tanh<N: AsRef<Node>>(x: N) -> Node {
    let mut status = Status::new();
    unsafe {
        let node = Node::from_inner_ptr(_primitiv::safe_primitiv_node_func_tanh(
            x.as_ref().as_inner_ptr(),
            status.as_inner_mut_ptr(),
        ));
        status.into_result().unwrap();
        node
    }
}

pub fn relu<N: AsRef<Node>>(x: N) -> Node {
    let mut status = Status::new();
    unsafe {
        let node = Node::from_inner_ptr(_primitiv::safe_primitiv_node_func_relu(
            x.as_ref().as_inner_ptr(),
            status.as_inner_mut_ptr(),
        ));
        status.into_result().unwrap();
        node
    }
}

pub fn softmax_cross_entropy<N: AsRef<Node>>(x: N, ids: &[u32], dim: u32) -> Node {
    let mut status = Status::new();
    unsafe {
        let node = Node::from_inner_ptr(
            _primitiv::safe_primitiv_node_func_softmax_cross_entropy_with_array(
                x.as_ref().as_inner_ptr(),
                ids.as_ptr() as *const _,
                ids.len(),
                dim,
                status.as_inner_mut_ptr(),
            ),
        );
        status.into_result().unwrap();
        node
    }
}

pub fn dropout<N: AsRef<Node>>(x: N, rate: f32, enabled: bool) -> Node {
    let mut status = Status::new();
    unsafe {
        let node = Node::from_inner_ptr(_primitiv::safe_primitiv_node_func_dropout(
            x.as_ref().as_inner_ptr(),
            rate,
            enabled as u8,
            status.as_inner_mut_ptr(),
        ));
        status.into_result().unwrap();
        node
    }
}

pub mod batch {
    use primitiv_sys as _primitiv;
    use Node;
    use Status;
    use Wrap;

    pub fn mean<N: AsRef<Node>>(x: N) -> Node {
        let mut status = Status::new();
        unsafe {
            let node = Node::from_inner_ptr(_primitiv::safe_primitiv_node_func_batch_mean(
                x.as_ref().as_inner_ptr(),
                status.as_inner_mut_ptr(),
            ));
            status.into_result().unwrap();
            node
        }
    }
}
