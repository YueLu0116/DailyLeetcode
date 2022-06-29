package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func (head *ListNode) insert(val int) {
	mover := head
	for ; mover.Next != nil; mover = mover.Next {
	}
	new_node := ListNode{}
	new_node.Val = val
	new_node.Next = nil
	mover.Next = &new_node
}

func (list *ListNode) print_list() {
	mover := list
	for ; mover != nil; mover = mover.Next {
		fmt.Printf("%d", mover.Val)
	}
	fmt.Println()
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	point1 := l1
	point2 := l2
	carrier := 0

	head := ListNode{}
	mover := &head

	for (point1 != nil) && (point2 != nil) {
		var tmp_sum = point1.Val + point2.Val + carrier

		tmp_node := ListNode{}
		tmp_node.Val = tmp_sum % 10
		mover.Next = &tmp_node
		mover = mover.Next

		carrier = tmp_sum / 10
		point1 = point1.Next
		point2 = point2.Next
	}

	for ; point1 != nil; point1 = point1.Next {
		var tmp_sum = point1.Val + carrier
		tmp_node := ListNode{}
		tmp_node.Val = tmp_sum % 10
		mover.Next = &tmp_node
		mover = mover.Next
		carrier = tmp_sum / 10
	}
	for ; point2 != nil; point2 = point2.Next {
		var tmp_sum = point2.Val + carrier
		tmp_node := ListNode{}
		tmp_node.Val = tmp_sum % 10
		mover.Next = &tmp_node
		mover = mover.Next
		carrier = tmp_sum / 10
	}

	if carrier != 0 {
		tmp_node := ListNode{}
		tmp_node.Val = carrier
		mover.Next = &tmp_node
	}

	return head.Next
}

func main() {
	vec1 := []int{9, 9, 9, 9, 9, 9, 9}
	vec2 := []int{9, 9, 9, 9}

	head1 := ListNode{}
	head2 := ListNode{}

	for _, val := range vec1 {
		head1.insert(val)
	}
	head1.Next.print_list()

	for _, val := range vec2 {
		head2.insert(val)
	}
	head2.Next.print_list()

	added_list := addTwoNumbers(head1.Next, head2.Next)
	added_list.print_list()
}
