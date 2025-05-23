package java.algorithms.algorithms_82;

import java.util.LinkedList;

public class linkedlists_86 {

//    LinkedList = stores Nodes in 2 parts(data + address)
//                 Nodes are in non-consecutive memory locations
//                 Elements are linked using pointer
//
//
//    Singly Linked List:
//    Node                    Node                Node
//    [data | address] -> [data | address] -> [data | address]

//    Double Linked List:
//    Node                                    Node
//    [address | data | address] <-> [address | data | address]
//
//    advantages?
//    1. Dynamic Storage (allocations needed memory while running)
//    2. Insertion and Deletion of Nodes is easy (I/O)
//    3. No/Low memory waste
//
//    disadvantages?
//    1. Greater memory usage (additional pointer)
//    2. No random acces of elements (no index [i]
//    3. Accessing/searching elements is more time consuming. O(n)
//
//    uses?
//    1. implements Stacks/Queues
//    2. GPS navigation
//    3. music playlist

    public static void main(String[] args) {

        LinkedList<String> linkedlist = new LinkedList<String>();

        //offer is also working
        linkedlist.push("A");
        linkedlist.push("B");
        linkedlist.push("C");
        linkedlist.push("D");
        linkedlist.push("F");

        //poll is also working
        //linkedlist.pop();

        linkedlist.add(1,"E");
        linkedlist.remove("E");

        System.out.println(linkedlist.peekFirst());
        System.out.println(linkedlist.peekLast());
        linkedlist.addFirst("G");
        linkedlist.addLast("0");

        String first = linkedlist.removeFirst();
        String last = linkedlist.removeLast();

        System.out.println(first + " " + last);
        System.out.println(linkedlist);
    }
}
