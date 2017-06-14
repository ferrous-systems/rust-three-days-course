// slice::split_at

fn split_at<T>(&[T], mid: usize) -> (&[T], &[T])

}

fn split_at<'a, T>(&'a [T], mid: usize) -> (&'a [T], &'a [T])

}