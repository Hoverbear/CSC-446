// EventList is implemented as a splay tree
// v1.  class Event knows this and has fields 
// leftlink, rightlink, uplink

class EventList {
    Event root;
    Event update_p;
    Event left,right,next,temp,farleft,farfarleft;
    double ntime;
    boolean update_code;
    int size = 0;
    Object owner;
		
    public EventList(){ }

    public String toString(){
      return "EventList ";
    }
		
    public boolean isEmpty(){
      return (this.root == null);
    }

    public void enqueue(Event n){
      size++;
      n.uplink = null;
      next = this.root;
      this.root = n;
      if (next == null){
	n.leftlink = null;
	n.rightlink = null;
      }
      else{
	mainblock: {
	  ntime = n.time;
	  left = n;
	  right = n;
	  if (next.time > ntime){
	    //2222222222222222222222222
	    do{
	      temp = next.leftlink;
	      if (temp == null){
		right.leftlink = next;
		next.uplink = right;
		left.rightlink = null;
		break mainblock;
	      }
	      if (temp.time <= ntime){
		right.leftlink = next;
		next.uplink = right;
		right = next;
		next = temp;
		break;						
	      }
	      next.leftlink = temp.rightlink;
	      if (temp.rightlink != null)
		temp.rightlink.uplink = next;
	      right.leftlink = temp;
	      temp.uplink = right;
	      temp.rightlink = next;
	      next.uplink = temp;
	      right = temp;
	      next = temp.leftlink;
	      if (next == null){
		left.rightlink = null;
		break mainblock;
	      }
	    }while (next.time > ntime);
	  }
	  forblock:
	  for( ; ; ){
	    //111111111111111111111111111
	    do{
	      temp = next.rightlink;
	      if (temp == null){
		left.rightlink = next;
		next.uplink = left;
		right.leftlink = null;
		break forblock;
	      }
	      if (temp.time > ntime){
		left.rightlink = next;
		next.uplink = left;
		left = next;
		next = temp;
		break;
	      }
	      next.rightlink = temp.leftlink;
	      if (temp.leftlink!=null)
		temp.leftlink.uplink = next;
	      left.rightlink = temp;
	      temp.uplink = left;
	      temp.leftlink = next;
	      next.uplink = temp;
	      left = temp;
	      next = temp.rightlink;
	      if (next == null){
		right.leftlink = null;
		break forblock;
	      }
	    }while (next.time <= ntime);
						
	    //2222222222222222222222222222
	    do{
	      temp = next.leftlink;
	      if (temp == null){
		right.leftlink = next;
		next.uplink = right;
		left.rightlink = null;
		break forblock;
	      }
	      if (temp.time <= ntime){
		right.leftlink = next;
		next.uplink = right;
		right = next;
		next = temp;
		break;
	      }
	      next.leftlink = temp.rightlink;
	      if (temp.rightlink != null)
		temp.rightlink.uplink = next;
	      right.leftlink = temp;
	      temp.uplink = right;
	      temp.rightlink = next;
	      next.uplink = temp;
	      right = temp;
	      next = temp.leftlink;
	      if (next == null){
		left.rightlink = null;
		break forblock;
	      }
	    }while (next.time > ntime);
	  }
	}
				
	//99999999999999999999999999999
	temp = n.leftlink;
	n.leftlink = n.rightlink;
	n.rightlink = temp;
      }
    }
    public Event getMin(){
			
      //assumed that the tree is not empty
      next = this.root;
      left = next.leftlink;
      if (left ==null){
	update_code = true;
	return next;
      }
      else{
	for ( ; ; ){
	  farleft = left.leftlink;
	  if (farleft == null){
	    update_code = false;
	    update_p = next;
	    return left;
	  }
	  farfarleft = farleft.leftlink;
	  if (farfarleft==null){
	    update_code = false;
	    update_p = left;
	    return farleft;
	  }
	  next.leftlink = farleft;
	  farleft.uplink = next;
	  left.leftlink = farleft.rightlink;
	  if (farleft.rightlink!=null)
	    farleft.rightlink.uplink = left;
	  farleft.rightlink = left;
	  left.uplink = farleft;
	  next = farleft;
	  left = farfarleft;
	}
      }
    }
		
    public void dequeue(){
      size--;
      //assumed that getMin has been the last splay tree operation invoked. Removes that element.
      if (update_code){
	next = null; //remove pointer to the returned Event
	root = root.rightlink;
	if (root!=null)
	  root.uplink = null;
      }
      else{
	left = farleft = null; //remove pointers to the returned event
	if (update_p.leftlink.rightlink!=null)
	  update_p.leftlink.rightlink.uplink = update_p;
	update_p.leftlink = update_p.leftlink.rightlink;
      }
    }
  }
