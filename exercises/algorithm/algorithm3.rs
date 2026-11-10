/*
	排序
	本题要求你实现一种排序算法
	你可以使用冒泡排序、插入排序、堆排序等
*/

// I AM NOT DONE

fn sort<T:Ord>(array: &mut [T]){
	for end in (1..array.len()).rev() {
        for i in 0..end {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}