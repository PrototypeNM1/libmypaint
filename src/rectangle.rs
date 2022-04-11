use std::{os::raw::c_int, ptr::null_mut};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MyPaintRectangle {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MyPaintRectangles {
    pub num_rectangles: c_int,
    pub rectangles: *mut MyPaintRectangle,
}

#[no_mangle]
pub unsafe extern "C" fn mypaint_rectangle_expand_to_include_point(
    r: *mut MyPaintRectangle,
    x: c_int,
    y: c_int,
) {
    if (*r).width == 0 {
        *r = MyPaintRectangle {
            x,
            y,
            width: 1,
            height: 1,
        };
    } else {
        let expand = |origin, extent, point| -> (c_int, c_int) {
            let p1 = std::cmp::min(origin, point);
            let mut p2 = origin + extent;
            p2 = std::cmp::max(p2, point+1); // +1 to enclose point
            (p1, p2 - p1)
        };

        ((*r).x, (*r).width) = expand((*r).x, (*r).width, x);
        ((*r).y, (*r).height) = expand((*r).y, (*r).height, y);
    }
}

#[no_mangle]
pub unsafe extern "C" fn mypaint_rectangle_expand_to_include_rect(
    r: *mut MyPaintRectangle,
    other: *const MyPaintRectangle,
) {
    mypaint_rectangle_expand_to_include_point(r, (*other).x, (*other).y);
    mypaint_rectangle_expand_to_include_point(r, (*other).x + (*other).width, (*other).y + (*other).height);
}

#[no_mangle]
pub unsafe extern "C" fn mypaint_rectangle_copy(
    source: *const MyPaintRectangle,
) -> *mut MyPaintRectangle {
    if source.is_null() {
        return null_mut();
    }

    let layout = core::alloc::Layout::new::<MyPaintRectangle>();
    let dest = std::alloc::alloc(layout) as *mut MyPaintRectangle;
    *dest = *source;
    dest
}
