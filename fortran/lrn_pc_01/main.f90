program main
    implicit none
    integer :: i,j
    integer, dimension(5, 5) :: nums
    integer, dimension(25) :: nums2
    integer, dimension(5, 5) :: nums3
    nums2 = (/(i, i=1, 25)/)
    write(*,*) nums2
    nums = reshape((/(i, i=1,25)/), (/5,5/))
    write(*,100) ((nums(i,j), j=1,5), i=1,5)
    nums3 = reshape((/((5*i+j, i=0,4), j=1,5)/), (/5,5/))
    write(*,100) ((nums3(i,j), j=1,5), i=1,5)
    100 format(1x, 5i8)
    write(*,*) (nums3(1:3, 1:5:2))
end program main