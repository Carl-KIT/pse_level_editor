fn stairs_orientation(start: (f32, f32), end: (f32, f32)) -> i32 {
    if (start.0 < end.0 && start.1 < end.1) {
        return 1;
    }
    else if (start.0 > end.0 && start.1 > end.1) {
        return 1;
    }

    return -1;
}