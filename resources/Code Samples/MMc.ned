//
// This file is part of an OMNeT++/OMNEST simulation example.
//
// Copyright (C) 2006-2008 OpenSim Ltd
//
// This file is distributed WITHOUT ANY WARRANTY. See the file
// `license' for details on this and other legal matters.
//

import org.omnetpp.queueing.Queue;
import org.omnetpp.queueing.Sink;
import org.omnetpp.queueing.Source;
import org.omnetpp.queueing.PassiveQueue;
import org.omnetpp.queueing.Server;


//
// This simple queueing network only contains a source, a FIFO queue and a sink.
//
network mmcQueue
{
    parameters:
        @display("i=block/network2;bgb=536,310");
    submodules:
        sink: Sink {
            @display("p=449,128");
        }
        queue: PassiveQueue {
            @display("p=165.0,79.0");
        }
        server1: Server {
            @display("p=315,63");
        }
        server2: Server {
            @display("p=315,155");
        }
        source: Source {
            @display("p=50.0,79.0");
        }
    connections:
        source.out --> queue.in++;
        queue.out++ --> server1.in++;
        queue.out++ --> server2.in++;
        server1.out --> sink.in++;
        server2.out --> sink.in++;
}
