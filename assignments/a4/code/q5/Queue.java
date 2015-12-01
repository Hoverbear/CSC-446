// package osl.util;
// import java.io.*;

/**

  This class provides a fairly efficient implementation of a FIFO
  queue.  We use an array to implement the queue which is
  automatically increased in size when necessary.  Note that the queue
  is never shrunk in the current implementation.

  @author Mark Astley
  @version $Revision: 1.5 $ ($Date: 1998/09/07 23:00:37 $)

  */

// public class Queue implements Cloneable, Serializable {
public class Queue {
  /**
    This constant sets the initial size of a queue when the default
    constructor is invoked to create a queue.  Users requiring a
    different initial size may invoke the appropriate constructor
    below.
    */
  public static final int INITIAL_SIZE = 10;

  /**
    This array holds the queue.  We use a dynamically growing circular
    array to represent the queue.  Queue size is doubled when
    necessary which should make for a rather efficient implementation.
    */
  Object[] theQ;

  /**
    These two fields hold indices to the first and last element of the
    queue respectively.  Technically, <em>back</em> points to the
    first open spot in the queue rather than the last element.
    */
  int front;
  int back;

  /**
    Default constructor for the <em>Queue</em> class.
    */
  public Queue() {
    theQ = new Object[INITIAL_SIZE];
    front=back=0;
  }

  /** 
    Alternative constructor for the <em>Queue</em> class which allows
    the specification of the initial size of the queue.

    @param <b>initialSize</b> The initial size of the new <em>Queue</em>.
    */
  public Queue(int initialSize) {
    theQ = new Object[initialSize];
    front=back=0;
  }

  /**
    This private method is used to double the size of the queue when
    more space is required (as a result of calling enqueue).  Note
    that because the implementation is in terms of a circular array,
    we have to be careful when copying elements from the old array to
    the new array.
    */
  synchronized private void grow() {
    Object[] newQ = new Object[2 * theQ.length];

    if (back >= front)
      System.arraycopy(theQ, front, newQ, front, back - front + 1);
    else {
      System.arraycopy(theQ, front, newQ, front, theQ.length - front);
      System.arraycopy(theQ, 0, newQ, theQ.length, back);
      back = front + theQ.length - 1;
    }

    theQ = newQ;
  }

  /**
    Determine if the queue is empty.

    @return <b>true</b> if the queue contains no elements,
    <b>false</b> otherwise.
    */
  synchronized public boolean empty() {
      return (front == back);
  }

  /**
    Add an object to the end of the queue.

    @param <b>q</b> A reference to the object to add.
    @return <b>void</b>
    */
  synchronized public void enqueue(Object q) {
    if (((back + 1) % theQ.length) == front)
      grow();
      
    theQ[back] = q;
    back = (back + 1) % theQ.length;
  }

  /**
    Remove an object from the front of the queue.  The returned object
    is removed from the queue.

    @return A reference to the <b>Object</b> at the front of the
    queue.  Returns <em>null</em> if the queue is empty.
    */
  synchronized public Object dequeue() {
    Object toReturn = null;

    if (!empty()) {
      toReturn = theQ[front];
      front = (front + 1) % theQ.length;
    }

    return toReturn;
  }

  /**
    Get the object at the front of the queue without removing it.

    @return A reference to the <b>Object</b> at the front of the
    queue.
    */
  synchronized public Object peekFront() {
    if (!empty())
      return theQ[front];
    else
      return null;
  }

  /**
    Determine how many objects are currently stored in the queue.

    @return An <b>int</b> indicating the number of objects in the
    queue.
    */
  synchronized public int numElements() {
    if (front <= back) 
      return (back - front);
    else 
      return (back + (theQ.length - front));
  }


  /**
    This function allows arbitrary queue elements to be removed.
    Elements which are equal (according to the <em>equal</em>
    function) to the argument are removed from the queue.  Note that
    this version of the function only removes the FIRST element
    found.  Use the <em>removeAll</em> function to remove ALL elements
    equal to the argument.

    @param <b>rem</b> The object to be removed from the queue.
    @return <b>true</b> if an element was removed, <b>false</b>
    otherwise.
    */
  synchronized public boolean remove(Object rem) {
    int i, x, y;
    
    // PRAGMA [debug,osl.util.Queue] Log.println("<Queue.remove> Searching for element: " + rem);
    // First track down the element to remove (if there is one)
    for(i=front; i != back; i = (i + 1) % theQ.length)
      if (rem.equals(theQ[i]))
	break;

    // If nothing found then return false...
    if (i == back)
      // PRAGMA [debug,osl.util.Queue] {
      // PRAGMA [debug,osl.util.Queue] Log.println("<Queue.remove> Element not found, returning");
      return false;
    // PRAGMA [debug,osl.util.Queue] } else {
    // PRAGMA [debug,osl.util.Queue] Log.println("<Queue.remove> Element found at position: " + i);
    // PRAGMA [debug,osl.util.Queue] }

    // Otherwise remove the one element.  This is a bit of a hack but
    // should be fairly efficient.
    Object[] newQ = new Object[theQ.length];
    for(x=front, y=0; x != back; x = (x + 1) % theQ.length) 
      if (x != i) {
	newQ[y] = theQ[x];
	y++;
      }

    theQ = newQ;
    front = 0;
    back = y;

    return true;
  }

  /**
    This function has the same behavior as <em>remove</em> except that
    ALL matching elements are removed from the queue.

    @param <b>rem</b> The object to be removed from the queue.
    @return The number of elements removed.
    */
  synchronized public int removeAll(Object rem) {
    int i, match, x, y, z;
    
    // Track down all the elements to remove.
    int[] toRemove = new int[numElements()];
    for(i=front, match=0; i != back; i = (i + 1) % theQ.length)
      if (rem.equals(theQ[i])) {
	toRemove[match] = i;
	match++;
      }

    // See if anything matched, if not then just return
    if (match == 0)
      return 0;

    // Now make a new array with all the matching elements removed
    Object[] newQ = new Object[theQ.length];
    for(x=front, y=0, z=0; x != back; x = (x + 1) % theQ.length) 
      if (z < match) 
	if (x != toRemove[z]) {
	  newQ[y] = theQ[x];
	  y++;
	} else 
	  z++;
      else {
	newQ[y] = theQ[x];
	y++;
      }

    theQ = newQ;
    front = 0;
    back = y;
	
    // match holds the number of things we removed
    return match;
  }

  /**
    The canonical toString method.
    */
  synchronized public String toString() {
    String returnVal = "Queue: ";

    if (empty())
      return returnVal + "no elements";

    for(int i=front; i != back; i = (i + 1) % theQ.length) 
      returnVal = returnVal + theQ[i].toString() + " ";

    return returnVal;
  }
}


